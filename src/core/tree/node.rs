use super::change::Change;
use crate::{
    core::{graphics::Graphics, Constraints, Layout, View},
    utils::id_vec::Id,
};
use std::rc::Rc;

pub struct Node {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub change: Change,

    pub view: Rc<dyn View>,
    pub constraints: Constraints,
    pub layout: Layout,
    pub graphics: Graphics,
}

impl Node {
    pub fn new(parent: Option<Id>, view: Rc<dyn View>) -> Self {
        Node {
            parent,
            children: Default::default(),
            change: Change::ALL,
            view,
            constraints: Default::default(),
            layout: Default::default(),
            graphics: Default::default(),
        }
    }
}
