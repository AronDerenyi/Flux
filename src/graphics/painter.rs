use super::{Color, Paint, Paragraph};
use crate::math::Vec2;
use skia_safe::{Canvas, Point, Vector};

pub struct Painter<'a> {
    canvas: &'a Canvas,
}

impl Painter<'_> {
    pub fn new<'a>(canvas: &'a Canvas) -> Painter<'a> {
        Painter { canvas }
    }

    pub fn translate(&mut self, translation: Vec2, f: impl FnOnce(&mut Painter)) {
        if translation == Vec2::ZERO {
            f(self);
        } else {
            self.canvas.save();
            self.canvas
                .translate(Vector::new(translation.x, translation.y));
            f(self);
            self.canvas.restore();
        }
    }

    pub fn draw_rect(&mut self, position: Vec2, size: Vec2, paint: impl Into<Paint>) {
        self.canvas.draw_rect(
            skia_safe::Rect::new(
                position.x,
                position.y,
                position.x + size.x,
                position.y + size.y,
            ),
            &paint.into().into(),
        );
    }

    pub fn draw_paragraph(&mut self, paragraph: &Paragraph, position: Vec2, width: f32) {
        let mut sk_paragraph = paragraph.sk_paragraph.borrow_mut();
        if sk_paragraph.max_width() != width {
            sk_paragraph.layout(width);
        }
        sk_paragraph.paint(self.canvas, Point::new(position.x, position.y));
    }
}

impl From<Paint> for skia_safe::Paint {
    fn from(paint: Paint) -> Self {
        let mut sk_paint = skia_safe::Paint::default();
        match paint {
            Paint::Fill { color } => {
                sk_paint.set_color(color);
            }
            Paint::Stroke { width, color } => {
                sk_paint.set_stroke(true);
                sk_paint.set_color(color);
                sk_paint.set_stroke_width(width);
                sk_paint.set_stroke_miter(10.0);
            }
        }
        sk_paint
    }
}

impl From<Color> for skia_safe::Color {
    fn from(color: Color) -> Self {
        skia_safe::Color::from_argb(color.a, color.r, color.g, color.b)
    }
}
