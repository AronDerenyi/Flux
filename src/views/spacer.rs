use crate::{
    core::{Constraints, ViewSize},
    View,
};
use macroquad::math::Vec2;

#[derive(PartialEq)]
pub struct Spacer {
    size: Vec2,
}

pub fn spacer(size: Vec2) -> Spacer {
    Spacer { size }
}

impl View for Spacer {
    fn size(&self, constraints: Constraints, children: Vec<ViewSize>) -> Vec2 {
        self.size
    }
}
