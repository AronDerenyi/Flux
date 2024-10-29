use super::change::Change;
use crate::{
    core::{graphics::Graphics, Constraints, View},
    utils::id_vec::Id,
};
use macroquad::math::Vec2;
use std::{collections::HashMap, rc::Rc};

pub struct Node {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub change: Change,

    pub view: Rc<dyn View>,
    pub cache: HashMap<Constraints, Vec2>, // TODO: cache cleaning (one idea is that any cached size who wasn't queried in the last pass can be thrown out)
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
            cache: Default::default(),
            position: Default::default(),
            size: Default::default(),
            graphics: Default::default(),
        }
    }
}

/*

After state changed:

Rebuild children (and children's children) if their view changed.
If the view changed, invalidate the size cache.

Validate children's size cache by checking if the cached sizes are equal to the generated sizes.
If any child's size cache still invalid, invalidate the parent size cache and repeat from the previous step.

*/
