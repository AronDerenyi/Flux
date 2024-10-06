use macroquad::{color::Color, math::Vec2};

pub enum Shape {
    Rect {
        position: Vec2,
        size: Vec2,
        fill: Option<Color>,
        stroke: Option<(f32, Color)>,
    },
}
