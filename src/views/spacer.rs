use crate::{core::Constraints, View};
use macroquad::math::Vec2;

#[derive(PartialEq)]
pub struct Spacer {
    size: Vec2,
}

pub fn spacer(size: Vec2) -> Spacer {
    Spacer { size }
}

impl View for Spacer {
    fn calculate_constraints(&self, _child_constraints: &[Constraints]) -> Constraints {
        Constraints { size: self.size }
    }
}
