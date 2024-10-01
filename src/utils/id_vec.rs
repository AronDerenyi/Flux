use std::ops::{Index, IndexMut};

pub struct IdVec<T> {
    nodes: Vec<Option<T>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Id(pub usize);

impl<T> IdVec<T> {
    pub fn new() -> Self {
        IdVec { nodes: vec![] }
    }

    pub fn insert(&mut self, value: T) -> Id {
        let id = Id(self.nodes.len());
        self.nodes.push(Some(value));
        id
    }

    pub fn remove(&mut self, id: Id) {
        self.nodes[id.0] = None;
    }
}

impl<T> Index<Id> for IdVec<T> {
    type Output = T;

    fn index(&self, id: Id) -> &Self::Output {
        if let Some(node) = &self.nodes[id.0] {
            node
        } else {
            panic!("Node not found: {}", id.0);
        }
    }
}

impl<T> IndexMut<Id> for IdVec<T> {
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        if let Some(node) = &mut self.nodes[id.0] {
            node
        } else {
            panic!("Node not found: {}", id.0);
        }
    }
}
