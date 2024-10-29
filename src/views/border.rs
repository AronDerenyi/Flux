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

    fn size(&self, constraints: Constraints, children: Vec<ViewSize>) -> Vec2 {
        if let Some(child) = children.into_iter().next() {
            child.size(constraints)
        } else {
            panic!("Border must have one child view")
        }
    }

    fn layout(&self, size: Vec2, children: Vec<ViewLayout>) {
        if let Some(child) = children.into_iter().next() {
            child.layout(Vec2::ZERO, size);
        }
    }

    fn draw(&self, size: Vec2, painter: &mut Painter) {
        painter.rect_stroke(Vec2::ZERO, size, self.width, self.color);
    }
}
