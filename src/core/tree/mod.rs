mod build;
mod change;
mod constraints;
mod debug;
mod graphics;
mod layout;
mod node;

use super::View;
use crate::utils::id_vec::{Id, IdVec};
use change::Change;
use itertools::Itertools;
pub use node::Node;
use std::{
    any::Any,
    collections::HashMap,
    ops::{Index, IndexMut},
    rc::Rc,
};

pub struct Tree {
    root: Id,
    nodes: IdVec<Node>,
}

impl Tree {
    pub fn new<V: View>(view: V) -> Self {
        let mut nodes = IdVec::new();
        let root = nodes.insert(Node::new(None, Rc::new(view)));
        Tree { root, nodes }
    }

    pub fn update(&mut self, states: &mut HashMap<Id, Rc<dyn Any>>, id: Id) {
        self.nodes[id].change.add(Change::VIEW);

        self.build(states, id);
        self.calculate_constraints(id);

        // TODO: Extract to a separate function
        let constraints_changed = self.nodes[id].change.contains(Change::CONSTRAINTS);
        let change_root = if constraints_changed {
            let mut change_root = self.nodes[id].parent;

            loop {
                let Some(id) = change_root else { break };

                let node = &self.nodes[id];
                let constraints = node.view.calculate_constraints(
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
                self.calculate_root_layout();
                self.root
            }
        } else {
            id
        };

        self.calculate_layouts(change_root);
        self.calculate_graphics(change_root);
        self.debug_print(self.root, "".into());
        self.reset(change_root);
    }

    pub fn traverse_down<F: FnMut(Id, &Node) -> bool>(&self, mut visitor: F) {
        self.traverse_down_from(self.root, &mut visitor);
    }

    fn traverse_down_from<F: FnMut(Id, &Node) -> bool>(&self, id: Id, visitor: &mut F) -> bool {
        let node = &self[id];
        if !visitor(id, node) {
            for child_id in node.children.clone() {
                if self.traverse_down_from(child_id, visitor) {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    pub fn traverse_up<F: FnMut(Id, &Node) -> bool>(&self, mut visitor: F) {
        self.traverse_down_from(self.root, &mut visitor);
    }

    fn traverse_up_from<F: FnMut(Id, &Node) -> bool>(&self, id: Id, visitor: &mut F) -> bool {
        let node = &self[id];
        for child_id in node.children.clone() {
            if self.traverse_down_from(child_id, visitor) {
                return true;
            }
        }
        visitor(id, node)
    }
}

impl Tree {
    fn insert(&mut self, parent: Id, view: Rc<dyn View>) -> Id {
        self.nodes.insert(Node::new(Some(parent), view.clone()))
    }

    fn remove(&mut self, id: Id) {
        for child_id in self[id].children.clone() {
            self.remove(child_id);
        }
        self.nodes.remove(id);
        // TODO: Clean up states and other id bound properties and callbacks
    }

    fn reset(&mut self, id: Id) {
        let node = &mut self.nodes[id];
        if !node.change.contains(Change::ALL) {
            return;
        }

        node.change.clear();
        for child_id in node.children.clone() {
            self.reset(child_id);
        }
    }
}

impl Index<Id> for Tree {
    type Output = Node;

    fn index(&self, id: Id) -> &Self::Output {
        &self.nodes[id]
    }
}

impl IndexMut<Id> for Tree {
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        &mut self.nodes[id]
    }
}
