use super::{Constraint, Constraints, Context, Interaction, Layout, Painter, View};
use crate::utils::id_vec::{Id, IdVec};
use itertools::{EitherOrBoth::*, Itertools};
use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};
use std::{any::Any, cell::RefCell, collections::HashMap, pin::Pin, rc::Rc};

pub struct ViewTree {
    root: Id,
    nodes: IdVec<RefCell<Node>>,
}

struct Node {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,

    pub view: Rc<dyn View>,
    pub layout: Layout,
}

impl ViewTree {
    pub fn new(root: impl View) -> Self {
        let mut nodes = IdVec::new();
        let root = nodes.insert(RefCell::new(Node {
            parent: None,
            children: Default::default(),
            view: Rc::new(root),
            layout: Layout {
                position: Default::default(),
                size: Default::default(),
            },
        }));

        ViewTree { root, nodes }
    }

    pub fn update(&mut self, states: &mut HashMap<Id, Rc<dyn Any>>, mut id: Id) {
        self.build(states, id);

        let size = ViewSizer {
            tree: self,
            id: self.root,
        }
        .size(Constraints {
            width: Constraint::Fixed(screen_width()),
            height: Constraint::Fixed(screen_height()),
        });

        self.layout(
            self.root,
            Layout {
                position: Vec2::ZERO,
                size,
            },
        );
    }

    pub fn draw(&self) {
        let mut painter = Painter::new(Vec2::ZERO);
        ViewDrawer {
            tree: self,
            id: self.root,
        }
        .draw(&mut painter);
    }

    pub fn interact(&self, interaction: Interaction) -> bool {
        ViewInteractor {
            tree: self,
            id: self.root,
        }
        .interact(interaction)
    }

    fn build(&mut self, states: &mut HashMap<Id, Rc<dyn Any>>, id: Id) {
        let node = self.nodes[id].borrow();
        let mut context = Context::new(id, states);
        let children = node.view.build(&mut context);
        let (paired_children, unused_children) =
            self.pair_children(children.into_iter(), node.children.iter().copied());

        drop(node);

        for id in unused_children {
            self.remove(id);
        }

        self.nodes[id].borrow_mut().children = paired_children
            .into_iter()
            .map(|(child_view, child_id)| {
                if let Some(child_id) = child_id {
                    let mut child_node = self.nodes[child_id].borrow_mut();
                    child_node.parent = Some(id);

                    // underlying struct is immutable (if no interior mutability is used)
                    // so if the references match the structs match too
                    if !Rc::ptr_eq(&child_node.view, &child_view) {
                        if *child_node.view != *child_view {
                            child_node.view = child_view;
                            drop(child_node);
                            self.build(states, child_id);
                        }
                    }
                    child_id
                } else {
                    let child_id = self.insert(id, child_view);
                    self.build(states, child_id);
                    child_id
                }
            })
            .collect();
    }

    fn remove(&mut self, id: Id) {
        let children = self.nodes[id].borrow().children.clone();
        for child_id in children {
            self.remove(child_id);
        }
        self.nodes.remove(id);
    }

    fn insert(&mut self, parent: Id, view: Rc<dyn View>) -> Id {
        self.nodes.insert(RefCell::new(Node {
            parent: Some(parent),
            children: Default::default(),
            view,
            layout: Layout {
                position: Default::default(),
                size: Default::default(),
            },
        }))
    }

    fn pair_children(
        &self,
        children: impl Iterator<Item = Rc<dyn View>>,
        child_indices: impl Iterator<Item = Id>,
    ) -> (Vec<(Rc<dyn View>, Option<Id>)>, Vec<Id>) {
        let mut paired_children = Vec::new();
        let mut unused_children = Vec::new();
        for pair in children.zip_longest(child_indices) {
            match pair {
                Both(child, id) => {
                    let old_child = self.nodes[id].borrow();
                    if old_child.view.type_id() == child.type_id() {
                        paired_children.push((child, Some(id)));
                    } else {
                        unused_children.push(id);
                        paired_children.push((child, None));
                    }
                }
                Left(child) => paired_children.push((child, None)),
                Right(index) => unused_children.push(index),
            }
        }
        (paired_children, unused_children)
    }

    fn layout(&self, id: Id, layout: Layout) {
        let mut node = self.nodes[id].borrow_mut();
        node.layout = layout;
        let layouts = node.view.layout(
            layout,
            &node
                .children
                .iter()
                .map(|&child_id| ViewSizer {
                    tree: self,
                    id: child_id,
                })
                .collect_vec(),
        );

        for (&child_id, layout) in node.children.iter().zip(layouts) {
            self.layout(child_id, layout);
        }
    }
}

pub struct ViewSizer<'a> {
    tree: &'a ViewTree,
    id: Id,
}

impl ViewSizer<'_> {
    pub fn size(&self, constraints: Constraints) -> Vec2 {
        let node = self.tree.nodes[self.id].borrow();
        node.view.size(
            constraints,
            &node
                .children
                .iter()
                .map(|&id| ViewSizer {
                    tree: self.tree,
                    id,
                })
                .collect_vec(),
        )
    }
}

pub struct ViewDrawer<'a> {
    tree: &'a ViewTree,
    id: Id,
}

impl ViewDrawer<'_> {
    pub fn draw(&self, painter: &mut Painter) {
        let node = self.tree.nodes[self.id].borrow();
        node.view.draw(
            node.layout,
            painter,
            &node
                .children
                .iter()
                .map(|&id| ViewDrawer {
                    tree: self.tree,
                    id,
                })
                .collect_vec(),
        )
    }
}

pub struct ViewInteractor<'a> {
    tree: &'a ViewTree,
    id: Id,
}

impl ViewInteractor<'_> {
    pub fn interact(&self, interaction: Interaction) -> bool {
        let node = self.tree.nodes[self.id].borrow();
        node.view.interact(
            node.layout,
            interaction,
            &node
                .children
                .iter()
                .map(|&id| ViewInteractor {
                    tree: self.tree,
                    id,
                })
                .collect_vec(),
        )
    }
}
