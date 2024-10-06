use std::rc::Rc;

use crate::core::{Constraints, Context, Layout, Shape, View, ViewBuilder};
use macroquad::color::Color;

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

    fn draw(&self, layout: Layout) -> Box<[Shape]> {
        Box::new([Shape::Rect {
            position: layout.position,
            size: layout.size,
            fill: None,
            stroke: Some((self.width, self.color)),
        }])
    }
}
