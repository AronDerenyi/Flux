use super::{
    view_node::{Shape, ViewNode},
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
    debug: Debug,
}

#[derive(Default)]
struct Debug {
    new: HashSet<Id>,
    rebuilt: HashSet<Id>,
    layout_recalculated: HashSet<Id>,
}

impl App {
    pub fn new<V: View>(root: V) -> Self {
        let mut nodes = IdVec::new();
        let root = nodes.insert(ViewNode::new(Rc::new(root), None));

        App {
            nodes,
            root,
            states: HashMap::new(),
            debug: Default::default(),
        }
    }

    pub fn update(&mut self, id: Id) {
        self.debug.new.clear();
        self.debug.rebuilt.clear();
        self.debug.layout_recalculated.clear();

        self.debug.rebuilt.insert(id);
        self.build_children(id);

        let node = &self.nodes[id];
        let constraints_changed = node
            .children
            .iter()
            .copied()
            .any(|id| self.nodes[id].layout.is_none());

        if constraints_changed {
            let mut id = id;
            let layout = loop {
                let prev_constraints = self.nodes[id].constraints;
                self.calculate_constraints(id);
                let node = &mut self.nodes[id];
                if node.constraints != prev_constraints {
                    node.layout = None;
                }
                if let Some(layout) = node.layout {
                    break layout;
                } else {
                    if let Some(parent) = node.parent {
                        id = parent;
                    } else {
                        break Layout {
                            position: Vec2::ZERO,
                            size: node.constraints.size,
                        };
                    }
                }
            };

            self.calculate_layouts(id, layout);
        } else {
            for child_id in node.children.clone() {
                let child_node = &self.nodes[child_id];
                self.calculate_layouts(child_id, child_node.layout.unwrap());
            }
        }
    }

    /// Sets the given view as the view of the id. If it's the same as the previous view,
    /// nothing changes. If there wasn't a previous view (the id is None) then an id is
    /// created for it. The children's layouts are invalidated and if the view's constraints
    /// changed then it's layout is also invalidated as it depends on the constraints.
    fn build(&mut self, parent: Id, view: Rc<dyn View>, id: Option<Id>) -> Id {
        let (id, prev_constraints) = if let Some(id) = id {
            let node = &mut self.nodes[id];
            node.parent = Some(parent);

            if Rc::ptr_eq(&node.view, &view) {
                // underlying struct is immutable (if no interior mutability is used)
                // so if the references match the structs match too
                return id;
            }

            node.view = view;
            node.dirty = true;
            self.debug.rebuilt.insert(id);
            (id, Some(node.constraints))
        } else {
            let id = self.nodes.insert(ViewNode::new(view, Some(parent)));
            self.debug.new.insert(id);
            (id, None)
        };

        self.build_children(id);
        self.calculate_constraints(id);

        if let Some(prev_constraints) = prev_constraints {
            let node = &mut self.nodes[id];
            if prev_constraints != node.constraints {
                node.layout = None;
            }
        }

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

    /// Calculates the constraints of the view with the given id.
    fn calculate_constraints(&mut self, id: Id) {
        let node = &self.nodes[id];
        self.nodes[id].constraints = node.view.get_constraints(
            &node
                .children
                .iter()
                .copied()
                .map(|id| self.nodes[id].constraints)
                .collect::<Box<_>>(),
        )
    }

    fn calculate_layouts(&mut self, id: Id, layout: Layout) {
        let node = &mut self.nodes[id];
        if let Some(prev_layout) = node.layout {
            if prev_layout == layout && !node.dirty {
                return;
            }
        }
        self.debug.layout_recalculated.insert(id);
        node.layout = Some(layout);

        // Only draw if the layout changed.
        // Drawing should be moved to a separate pass
        // Change tracing should be handled more comprehensively
        // Change should be a typed u8 (view: 0b01, layout: 0b10, view | layout: 0b11)
        node.graphics = node.view.draw(layout);
        node.dirty = false;

        let node = &self.nodes[id];
        let child_ids = node.children.clone();
        let child_layouts = node.view.get_children_layouts(
            layout,
            &child_ids
                .iter()
                .map(|child_id| self.nodes[*child_id].constraints)
                .collect::<Box<_>>(),
        );

        for (child_id, child_layout) in child_ids.iter().zip(child_layouts) {
            self.calculate_layouts(*child_id, child_layout);
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

        if let Some(layout) = node.layout {
            if layout.contains(point) {
                node.view.interact();
            }
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
            "{}({:?}): {}, {:?}{}{}{}",
            node.view.debug_name(),
            id,
            size_of_val(&*node.view),
            node.layout,
            if self.debug.rebuilt.contains(&id) {
                " rebuilt"
            } else {
                ""
            },
            if self.debug.new.contains(&id) {
                " new"
            } else {
                ""
            },
            if self.debug.layout_recalculated.contains(&id) {
                " layout"
            } else {
                ""
            }
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
