use super::change::Change;
use crate::{
    core::{graphics::Graphics, Constraints, Position, Size, SizeHint, View},
    utils::id_vec::Id,
};
use std::rc::Rc;

pub struct Node {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub change: Change,

    pub view: Rc<dyn View>,
    pub size_hint: SizeHint,
    pub constraints: Constraints,
    pub size: Size,
    pub position: Position,
    pub graphics: Graphics,
}

impl Node {
    pub fn new(parent: Option<Id>, view: Rc<dyn View>) -> Self {
        Node {
            parent,
            children: Default::default(),
            change: Change::ALL,

            view,
            size_hint: Default::default(),
            constraints: Default::default(),
            size: Default::default(),
            position: Default::default(),
            graphics: Default::default(),
        }
    }
}
