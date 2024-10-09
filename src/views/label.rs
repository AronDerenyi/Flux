use crate::core::{Constraints, Painter, Position, Size, SizeHint, View};
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
    fn calculate_size_hint(&self, child_size_hints: &[SizeHint]) -> SizeHint {
        let width = measure_text(&self.text, None, (self.size * 2.0) as u16, 1.0).width;
        let height = self.size * 2.0;
        SizeHint {
            min_width: width,
            min_height: height,
            ideal_width: width,
            ideal_height: height,
            max_width: width,
            max_height: height,
        }
    }

    fn calculate_constraints(
        &self,
        constraints: Constraints,
        child_size_hints: &[SizeHint],
    ) -> Vec<Constraints> {
        vec![constraints]
    }

    fn calculate_layout(
        &self,
        constraints: Constraints,
        child_sizes: &[Size],
    ) -> (Size, Vec<Position>) {
        let width = measure_text(&self.text, None, (self.size * 2.0) as u16, 1.0).width;
        let height = self.size * 2.0;
        (Size { width, height }, Vec::new())
    }

    fn draw(&self, size: Size, painter: &mut Painter) {
        painter.text(
            self.text.clone(),
            (0.0, self.size * 1.5).into(),
            self.size,
            self.color,
        );
    }
}
