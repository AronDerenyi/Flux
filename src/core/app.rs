use super::{
    view_node::{Change, Shape, ViewNode},
    Constraints, Context, Layout,
};
use crate::{
    utils::id_vec::{Id, IdVec},
    View,
};
use itertools::{EitherOrBoth, Itertools};
use macroquad::{
    color::RED,
    math::Vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
};
use std::{
    any::{type_name_of_val, Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct App {
    nodes: IdVec<ViewNode>,
    root: Id,
    states: HashMap<Id, Rc<dyn Any>>,
}

impl App {
    pub fn new<V: View>(root: V) -> Self {
        let mut nodes = IdVec::new();
        let root = nodes.insert(ViewNode::new(Rc::new(root), None));

        App {
            nodes,
            root,
            states: HashMap::new(),
        }
    }

    pub fn update(&mut self, id: Id) {
        self.nodes[id].change.add(Change::VIEW);

        self.build_children(id);
        let constraints_changed = self.calculate_constraints(id).is_changed();

        let change_root = if constraints_changed {
            let mut change_root = self.nodes[id].parent;

            loop {
                let Some(id) = change_root else { break };

                let node = &self.nodes[id];
                let constraints = node.view.get_constraints(
                    &node
                        .children
                        .iter()
                        .map(|i| self.nodes[*i].constraints)
                        .collect_vec(),
                );

                let node = &mut self.nodes[id];
                node.change.add(Change::CHILD_CONSTRAINTS);
                if constraints != node.constraints {
                    node.constraints = constraints;
                    node.change.add(Change::CONSTRAINTS);
                    change_root = node.parent;
                } else {
                    break;
                }
            }

            if let Some(id) = change_root {
                id
            } else {
                let node = &mut self.nodes[self.root];
                let layout = Layout {
                    position: Vec2::ZERO,
                    size: node.constraints.size,
                };
                if layout != node.layout {
                    node.layout = layout;
                    node.change.add(Change::LAYOUT);
                }
                self.root
            }
        } else {
            id
        };

        self.calculate_layouts(change_root);
        self.calculate_graphics(change_root);
        self.print();
        self.reset_changes(change_root);
    }

    fn build(&mut self, parent: Id, view: Rc<dyn View>, id: Option<Id>) -> Id {
        let id = if let Some(id) = id {
            let node = &mut self.nodes[id];
            node.parent = Some(parent);

            if Rc::ptr_eq(&node.view, &view) {
                // underlying struct is immutable (if no interior mutability is used)
                // so if the references match the structs match too
                return id;
            }

            node.view = view;
            node.change.add(Change::VIEW);
            id
        } else {
            self.nodes.insert(ViewNode::new(view, Some(parent)))
        };

        self.build_children(id);
        id
    }

    /// Builds the children of the view with the given id.
    fn build_children(&mut self, id: Id) {
        let node = &self.nodes[id];
        let mut context = Context::new(id, &mut self.states);
        let children = node.view.get_children(&mut context);
        let (paired_children, unused_children) = self.pair_children(
            children.into_vec().into_iter(),
            node.children.iter().copied(),
        );

        for id in unused_children {
            self.remove(id);
        }

        self.nodes[id].children = paired_children
            .into_iter()
            .map(|(child_view, child_id)| self.build(id, child_view, child_id))
            .collect();
    }

    fn calculate_constraints(&mut self, id: Id) -> Changing<Constraints> {
        let node = &self.nodes[id];
        if !node.change.contains(Change::VIEW) {
            return Changing::Unchanged(node.constraints);
        }

        let child_ids = node.children.clone().into_vec();
        let mut child_constraints_changed = false;
        let child_constraints = child_ids
            .into_iter()
            .map(|id| {
                let constraints = self.calculate_constraints(id);
                child_constraints_changed |= constraints.is_changed();
                constraints.get()
            })
            .collect::<Vec<_>>();

        let node = &mut self.nodes[id];
        let constraints = node.view.get_constraints(&child_constraints);

        if child_constraints_changed {
            node.change.add(Change::CHILD_CONSTRAINTS);
        }

        if constraints != node.constraints {
            node.constraints = constraints;
            node.change.add(Change::CONSTRAINTS);
            Changing::Changed(constraints)
        } else {
            Changing::Unchanged(constraints)
        }
    }

    fn calculate_layouts(&mut self, id: Id) {
        let node = &self.nodes[id];
        if !node
            .change
            .contains(Change::VIEW | Change::CHILD_CONSTRAINTS | Change::LAYOUT)
        {
            return;
        }

        let child_ids = node.children.clone();
        let child_constraints = child_ids
            .iter()
            .map(|child_id| self.nodes[*child_id].constraints)
            .collect::<Vec<_>>();

        let child_layouts = node
            .view
            .get_children_layouts(node.layout, &child_constraints);

        for (child_id, child_layout) in child_ids.iter().zip(child_layouts) {
            let child_node = &mut self.nodes[*child_id];
            if child_layout != child_node.layout {
                child_node.layout = child_layout;
                child_node.change.add(Change::LAYOUT);
            }
            self.calculate_layouts(*child_id);
        }
    }

    /// Pairs up new children with their previous ids based on their position
    /// relative to the parent.
    fn pair_children(
        &self,
        children: impl Iterator<Item = Rc<dyn View>>,
        child_indices: impl Iterator<Item = Id>,
    ) -> (Vec<(Rc<dyn View>, Option<Id>)>, Vec<Id>) {
        let mut paired_children = Vec::new();
        let mut unused_children = Vec::new();
        for pair in children.zip_longest(child_indices) {
            match pair {
                EitherOrBoth::Both(child, id) => {
                    let old_child = &self.nodes[id];
                    if old_child.view.type_id() == child.type_id() {
                        paired_children.push((child, Some(id)));
                    } else {
                        unused_children.push(id);
                        paired_children.push((child, None));
                    }
                }
                EitherOrBoth::Left(child) => paired_children.push((child, None)),
                EitherOrBoth::Right(index) => unused_children.push(index),
            }
        }
        (paired_children, unused_children)
    }

    fn remove(&mut self, id: Id) {
        for child_id in self.nodes[id].children.clone() {
            self.remove(child_id);
        }
        self.nodes.remove(id);
        // TODO: Clean up states and other id bound properties and callbacks
    }

    fn calculate_graphics(&mut self, id: Id) {
        let node = &mut self.nodes[id];
        if !node.change.contains(Change::ALL) {
            return;
        }

        if node.change.contains(Change::VIEW | Change::LAYOUT) {
            node.graphics = node.view.draw(node.layout);
        }

        for child_id in node.children.clone() {
            self.calculate_graphics(child_id);
        }
    }

    fn reset_changes(&mut self, id: Id) {
        let node = &mut self.nodes[id];
        if !node.change.contains(Change::ALL) {
            return;
        }

        node.change.clear();
        for child_id in node.children.clone() {
            self.reset_changes(child_id);
        }
    }

    pub fn draw(&self) {
        self.draw_node(self.root);
    }

    fn draw_node(&self, id: Id) {
        let node = &self.nodes[id];

        for shape in node.graphics.iter() {
            match shape {
                Shape::Rect {
                    position,
                    size,
                    fill,
                    stroke,
                } => {
                    if let Some(color) = fill {
                        draw_rectangle(position.x, position.y, size.x, size.y, *color)
                    }
                    if let Some((width, color)) = stroke {
                        draw_rectangle_lines(position.x, position.y, size.x, size.y, *width, *color)
                    }
                }
            }
        }

        for child_id in node.children.iter() {
            self.draw_node(*child_id);
        }
    }

    pub fn interact(&self, point: Vec2) {
        self.interact_node(self.root, point);
    }

    fn interact_node(&self, id: Id, point: Vec2) {
        let node = &self.nodes[id];

        if node.layout.contains(point) {
            node.view.interact();
        }

        for child_id in node.children.iter() {
            self.interact_node(*child_id, point);
        }
    }

    pub fn print(&self) {
        self.print_node(self.root, "".into());
    }

    fn print_node(&self, id: Id, indent: String) {
        let node = &self.nodes[id];

        println!(
            "{}({:?}): {}, {:?}, {:?}",
            node.view.debug_name(),
            id,
            size_of_val(&*node.view),
            node.change,
            node.constraints.size
        );
        for (index, child_index) in node.children.iter().enumerate() {
            let last = index == node.children.len() - 1;
            print!("{}{} ", indent, if last { "╚" } else { "╠" });
            self.print_node(
                *child_index,
                if last {
                    indent.clone() + "  ".into()
                } else {
                    indent.clone() + "║ ".into()
                },
            );
        }
    }
}

enum Changing<T = ()> {
    Changed(T),
    Unchanged(T),
}

impl<T> Changing<T> {
    fn get(self) -> T {
        match self {
            Self::Changed(value) => value,
            Self::Unchanged(value) => value,
        }
    }

    fn is_changed(&self) -> bool {
        match self {
            Self::Changed(_) => true,
            Self::Unchanged(_) => false,
        }
    }
}
