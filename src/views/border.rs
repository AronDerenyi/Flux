use crate::core::{Constraints, Context, Layout, Painter, View, ViewBuilder};
use macroquad::color::Color;
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

    fn calculate_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        child_constraints[0]
    }

    fn calculate_layouts(&self, layout: Layout, _child_constraints: &[Constraints]) -> Vec<Layout> {
        vec![layout]
    }

    fn draw(&self, layout: Layout, painter: &mut Painter) {
        painter.rect_stroke(layout.position, layout.size, self.width, self.color);
    }
}
