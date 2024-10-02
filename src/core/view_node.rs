use super::{Constraints, Layout};
use crate::{utils::id_vec::Id, views::Spacer, View};
use macroquad::{color::Color, math::Vec2};
use std::{
    any::Any,
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct ViewNode {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub view: Rc<dyn View>,
    pub dirty: bool,
    pub constraints: Constraints,
    pub layout: Option<Layout>,
    pub graphics: Box<[Shape]>,
}

impl ViewNode {
    pub fn new(view: Rc<dyn View>, parent: Option<Id>) -> Self {
        ViewNode {
            parent,
            children: Default::default(),
            view,
            dirty: true,
            constraints: Default::default(),
            layout: Default::default(),
            graphics: Default::default(),
        }
    }
}

pub enum Shape {
    Rect {
        position: Vec2,
        size: Vec2,
        fill: Option<Color>,
        stroke: Option<(f32, Color)>,
    },
}
