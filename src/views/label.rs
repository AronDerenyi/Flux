use macroquad::{
    color::{Color, BLACK},
    math::Vec2,
    text::{draw_multiline_text, measure_text},
};

use crate::core::{Constraints, Layout, Painter, View};

#[derive(PartialEq)]
pub struct Label {
    text: String,
    size: f32,
    color: Color,
}

pub fn label(text: impl Into<String>) -> Label {
    Label {
        text: text.into(),
        size: 12.0,
        color: BLACK,
    }
}

impl Label {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for Label {
    fn calculate_constraints(&self, _child_constraints: &[Constraints]) -> Constraints {
        let measurements = measure_text(&self.text, None, (self.size * 2.0) as u16, 1.0);
        Constraints {
            size: Vec2::new(measurements.width, self.size * 2.0),
        }
    }

    fn draw(&self, layout: Layout, painter: &mut Painter) {
        painter.text(
            self.text.clone(),
            (layout.position.x, layout.position.y + self.size * 1.5).into(),
            self.size,
            self.color,
        );
    }
}
