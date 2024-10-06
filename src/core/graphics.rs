use macroquad::{color::Color, math::Vec2};

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

    pub fn draw(&self) {
        for shape in self.shapes.iter() {
            shape.draw();
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
}

impl Shape {
    fn draw(&self) {
        match self {
            Shape::Rect {
                position,
                size,
                fill,
                stroke,
            } => {
                if let Some(color) = fill {
                    macroquad::shapes::draw_rectangle(
                        position.x, position.y, size.x, size.y, *color,
                    )
                }
                if let Some((width, color)) = stroke {
                    macroquad::shapes::draw_rectangle_lines(
                        position.x, position.y, size.x, size.y, *width, *color,
                    )
                }
            }
        }
    }
}
