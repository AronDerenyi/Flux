use crate::core::{Constraints, Context, Painter, Position, Size, SizeHint, View, ViewBuilder};
use macroquad::{color::Color, math::Vec2};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Border {
    width: f32,
    color: Color,
    view: ViewBuilder,
}

pub trait Borderable: View + Sized {
    fn border(self, width: f32, color: Color) -> Border {
        Border {
            width,
            color,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Borderable for V {}

impl View for Border {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![self.view.build()]
    }

    fn calculate_size_hint(&self, child_size_hints: &[SizeHint]) -> SizeHint {
        child_size_hints[0]
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
        (child_sizes[0], vec![Position::default()])
    }

    fn draw(&self, size: Size, painter: &mut Painter) {
        painter.rect_stroke(
            Vec2::ZERO,
            Vec2::new(size.width, size.height),
            self.width,
            self.color,
        );
    }
}
