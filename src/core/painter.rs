use macroquad::{color::Color, math::Vec2, text::TextParams};

pub struct Painter {
    origin: Vec2,
}

impl Painter {
    pub fn new(origin: Vec2) -> Self {
        Self { origin }
    }

    pub fn translate(&mut self, translation: Vec2, func: impl FnOnce(&mut Painter)) {
        let mut painter = Painter::new(self.origin + translation);
        (func)(&mut painter);
    }

    pub fn rect_filled(&mut self, position: Vec2, size: Vec2, color: Color) {
        macroquad::shapes::draw_rectangle(
            position.x + self.origin.x,
            position.y + self.origin.y,
            size.x,
            size.y,
            color,
        )
    }

    pub fn rect_stroke(&mut self, position: Vec2, size: Vec2, width: f32, color: Color) {
        macroquad::shapes::draw_rectangle_lines(
            position.x + self.origin.x,
            position.y + self.origin.y,
            size.x,
            size.y,
            width,
            color,
        )
    }

    pub fn text(&mut self, text: &str, position: Vec2, size: f32, color: Color) {
        macroquad::text::draw_text_ex(
            text,
            position.x + self.origin.x,
            position.y + self.origin.y,
            TextParams {
                font: None,
                font_size: (size * 2.0) as u16,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
        );
    }
}
