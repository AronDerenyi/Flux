use super::ViewBuilder;
use crate::core::{Context, Painter, View, ViewLayout, ViewSize};
use macroquad::{color::Color, math::Vec2};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Background {
    color: Color,
    view: ViewBuilder,
}

pub trait Backgroundable: View + Sized {
    fn background(self, color: Color) -> Background {
        Background {
            color,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Backgroundable for V {}

impl View for Background {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![self.view.build()]
    }

    fn size(&self, constraints: Vec2, children: &[ViewSize]) -> Vec2 {
        children[0].size(constraints)
    }

    fn layout(&self, size: Vec2, children: &[ViewLayout]) {
        children[0].layout(Vec2::ZERO, size);
    }

    fn draw(&self, size: Vec2, painter: &mut Painter) {
        painter.rect_filled(Vec2::ZERO, size, self.color);
    }
}
