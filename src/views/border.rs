use super::ViewBuilder;
use crate::core::{Constraints, Context, Painter, View, ViewLayout, ViewSize};
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

    fn size(&self, constraints: Constraints, children: &[ViewSize]) -> Vec2 {
        children[0].size(constraints)
    }

    fn layout(&self, size: Vec2, children: &[ViewLayout]) {
        children[0].layout(Vec2::ZERO, size);
    }

    fn draw(&self, size: Vec2, painter: &mut Painter) {
        painter.rect_stroke(Vec2::ZERO, size, self.width, self.color);
    }
}
