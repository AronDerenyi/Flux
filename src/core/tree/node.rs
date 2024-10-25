use super::change::Change;
use crate::{
    core::{graphics::Graphics, View},
    utils::id_vec::Id,
};
use macroquad::math::Vec2;
use std::rc::Rc;

pub struct Node {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub change: Change,

    pub view: Rc<dyn View>,
    pub position: Vec2,
    pub size: Vec2,
    pub graphics: Graphics,
}

impl Node {
    pub fn new(parent: Option<Id>, view: Rc<dyn View>) -> Self {
        Node {
            parent,
            children: Default::default(),
            change: Change::ALL,
            view,
            position: Default::default(),
            size: Default::default(),
            graphics: Default::default(),
        }
    }
}
