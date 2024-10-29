use crate::core::{Constraints, Painter, View, ViewSize};
use macroquad::{
    color::{Color, BLACK},
    math::Vec2,
    text::{draw_multiline_text, measure_text},
};

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
    fn size(&self, constraints: Constraints, children: Vec<ViewSize>) -> Vec2 {
        let measurements = measure_text(&self.text, None, (self.size * 2.0) as u16, 1.0);
        Vec2::new(measurements.width, self.size * 2.0)
    }

    fn draw(&self, size: Vec2, painter: &mut Painter) {
        painter.text(
            self.text.clone(),
            Vec2::new(0.0, 0.0 + self.size * 1.5),
            self.size,
            self.color,
        );
    }
}
