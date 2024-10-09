mod build;
mod change;
mod constraints;
mod debug;
mod graphics;
mod layout;
mod node;
mod size_hints;

use super::{Position, View};
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
        self.calculate_size_hints(id);

        // TODO: Extract to a separate function
        let size_hint_changed = self.nodes[id].change.contains(Change::SIZE_HINT);
        let change_root = if size_hint_changed {
            let mut change_root = self.nodes[id].parent;

            loop {
                let Some(id) = change_root else { break };

                let node = &self.nodes[id];
                let size_hint = node.view.calculate_size_hint(
                    &node
                        .children
                        .iter()
                        .map(|i| self.nodes[*i].size_hint)
                        .collect_vec(),
                );

                let node = &mut self.nodes[id];
                node.change.add(Change::CHILD_SIZE_HINT);
                if size_hint != node.size_hint {
                    node.size_hint = size_hint;
                    node.change.add(Change::SIZE_HINT);
                    change_root = node.parent;
                } else {
                    break;
                }
            }

            if let Some(id) = change_root {
                id
            } else {
                self.calculate_root_constraints();
                self.root
            }
        } else {
            id
        };

        self.calculate_constraints(change_root);
        self.calculate_layouts(change_root);

        // TODO: Extract to a separate function
        let size_changed = self.nodes[change_root].change.contains(Change::SIZE);
        let change_root = if size_changed {
            let mut change_root = self.nodes[change_root].parent;

            loop {
                let Some(id) = change_root else { break };

                let node = &self.nodes[id];
                let child_ids = node.children.clone();
                let (size, child_positions) = node.view.calculate_layout(
                    node.constraints,
                    &child_ids.iter().map(|i| self.nodes[*i].size).collect_vec(),
                );

                for (child_id, child_position) in child_ids.iter().zip(child_positions) {
                    let child_node = &mut self[*child_id];
                    child_node.position = child_position;
                }

                let node = &mut self.nodes[id];
                node.change.add(Change::CHILD_SIZE);
                if size != node.size {
                    node.size = size;
                    node.change.add(Change::SIZE);
                    change_root = node.parent;
                } else {
                    break;
                }
            }

            if let Some(id) = change_root {
                id
            } else {
                let id = self.root;
                let root_node = &mut self[id];
                root_node.position = Position::default();
                self.root
            }
        } else {
            change_root
        };

        self.calculate_graphics(change_root);
        self.debug_print(self.root, "".into());
        self.reset(change_root);
    }

    pub fn traverse_down<F: FnMut(Position, Id, &Node) -> bool>(&self, mut visitor: F) {
        self.traverse_down_from(Position { x: 0.0, y: 0.0 }, self.root, &mut visitor);
    }

    fn traverse_down_from<F: FnMut(Position, Id, &Node) -> bool>(
        &self,
        origin: Position,
        id: Id,
        visitor: &mut F,
    ) -> bool {
        let node = &self[id];
        let origin = Position {
            x: origin.x + node.position.x,
            y: origin.y + node.position.y,
        };
        if !visitor(origin, id, node) {
            for child_id in node.children.clone() {
                if self.traverse_down_from(origin, child_id, visitor) {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    pub fn traverse_up<F: FnMut(Position, Id, &Node) -> bool>(&self, mut visitor: F) {
        self.traverse_down_from(Position { x: 0.0, y: 0.0 }, self.root, &mut visitor);
    }

    fn traverse_up_from<F: FnMut(Position, Id, &Node) -> bool>(
        &self,
        origin: Position,
        id: Id,
        visitor: &mut F,
    ) -> bool {
        let node = &self[id];
        let origin = Position {
            x: origin.x + node.position.x,
            y: origin.y + node.position.y,
        };
        for child_id in node.children.clone() {
            if self.traverse_up_from(origin, child_id, visitor) {
                return true;
            }
        }
        visitor(origin, id, node)
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
