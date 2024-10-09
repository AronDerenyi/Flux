use crate::{
    core::{Constraints, Position, Size, SizeHint},
    View,
};
use macroquad::math::Vec2;

#[derive(PartialEq)]
pub struct Spacer {
    width: f32,
    height: f32,
}

pub fn spacer() -> Spacer {
    Spacer {
        width: f32::INFINITY,
        height: f32::INFINITY,
    }
}

impl Spacer {
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl View for Spacer {
    fn calculate_size_hint(&self, child_size_hints: &[SizeHint]) -> SizeHint {
        SizeHint {
            min_width: 0.0,
            min_height: 0.0,
            ideal_width: 0.0,
            ideal_height: 0.0,
            max_width: self.width,
            max_height: self.height,
        }
    }

    fn calculate_layout(
        &self,
        constraints: Constraints,
        child_sizes: &[Size],
    ) -> (Size, Vec<Position>) {
        (
            Size {
                width: constraints.max_width,
                height: constraints.max_height,
            },
            Vec::default(),
        )
    }
}
