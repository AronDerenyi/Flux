use macroquad::{
    color::Color,
    math::Vec2,
    text::{draw_multiline_text_ex, draw_text, draw_text_ex, TextParams},
};

pub struct Painter {
    shapes: Vec<Shape>,
}

impl Painter {
    pub(super) fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn rect(
        &mut self,
        position: Vec2,
        size: Vec2,
        fill: Option<Color>,
        stroke: Option<(f32, Color)>,
    ) {
        self.shapes.push(Shape::Rect {
            position,
            size,
            fill,
            stroke,
        });
    }

    pub fn rect_filled(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.rect(position, size, Some(color), None);
    }

    pub fn rect_stroke(&mut self, position: Vec2, size: Vec2, width: f32, color: Color) {
        self.rect(position, size, None, Some((width, color)));
    }

    pub fn text(&mut self, text: String, position: Vec2, size: f32, color: Color) {
        self.shapes.push(Shape::Text {
            text,
            position,
            size,
            color,
        });
    }
}

#[derive(Default)]
pub struct Graphics {
    shapes: Box<[Shape]>,
}

impl Graphics {
    pub fn from_painter(painter: Painter) -> Self {
        Self {
            shapes: painter.shapes.into(),
        }
    }

    pub fn draw(&self, origin: Vec2) {
        for shape in self.shapes.iter() {
            shape.draw(origin);
        }
    }
}

enum Shape {
    Rect {
        position: Vec2,
        size: Vec2,
        fill: Option<Color>,
        stroke: Option<(f32, Color)>,
    },
    Text {
        text: String,
        position: Vec2,
        size: f32,
        color: Color,
    },
}

impl Shape {
    fn draw(&self, origin: Vec2) {
        match self {
            Shape::Rect {
                position,
                size,
                fill,
                stroke,
            } => {
                if let Some(color) = fill {
                    macroquad::shapes::draw_rectangle(
                        position.x + origin.x,
                        position.y + origin.y,
                        size.x,
                        size.y,
                        *color,
                    )
                }
                if let Some((width, color)) = stroke {
                    macroquad::shapes::draw_rectangle_lines(
                        position.x + origin.x,
                        position.y + origin.y,
                        size.x,
                        size.y,
                        *width,
                        *color,
                    )
                }
            }
            Shape::Text {
                text,
                position,
                size,
                color,
            } => {
                draw_text_ex(
                    text,
                    position.x + origin.x,
                    position.y + origin.y,
                    TextParams {
                        font: None,
                        font_size: (*size * 2.0) as u16,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                        rotation: 0.0,
                        color: *color,
                    },
                );
            }
        }
    }
}
