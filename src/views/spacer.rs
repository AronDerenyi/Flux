use crate::{core::Constraints, View};
use macroquad::math::Vec2;

#[derive(Clone)]
pub struct Spacer {
    size: Vec2,
}

impl Spacer {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

impl View for Spacer {
    fn get_constraints(&self, _child_constraints: &[Constraints]) -> Constraints {
        Constraints { size: self.size }
    }
}
