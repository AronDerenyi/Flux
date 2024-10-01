use super::{Constraints, Layout};
use crate::{utils::id_vec::Id, views::Spacer, View};
use macroquad::math::Vec2;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct ViewNode {
    pub view: Box<dyn View>,
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub constraints: Constraints,
    pub layout: Option<Layout>,
    pub dirty: bool,
}

impl ViewNode {
    pub fn new(view: Box<dyn View>, parent: Option<Id>) -> Self {
        ViewNode {
            view,
            parent,
            children: Default::default(),
            constraints: Default::default(),
            layout: Default::default(),
            dirty: true,
        }
    }
}
