use super::ViewBuilder;
use crate::{
    core::{Constraints, Context, ViewLayout, ViewSize},
    View,
};
use macroquad::math::Vec2;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Padding {
    start: f32,
    end: f32,
    top: f32,
    bottom: f32,
    view: ViewBuilder,
}

#[allow(unused)]
pub trait Paddable: View + Sized {
    fn padding(self, start: f32, end: f32, top: f32, bottom: f32) -> Padding {
        Padding {
            start,
            end,
            top,
            bottom,
            view: ViewBuilder::from_view(self),
        }
    }

    fn padding_all(self, padding: f32) -> Padding {
        self.padding(padding, padding, padding, padding)
    }

    fn padding_axial(self, horizontal: f32, vertical: f32) -> Padding {
        self.padding(horizontal, horizontal, vertical, vertical)
    }

    fn padding_horizontal(self, horizontal: f32) -> Padding {
        self.padding(horizontal, horizontal, 0.0, 0.0)
    }

    fn padding_vertical(self, vertical: f32) -> Padding {
        self.padding(0.0, 0.0, vertical, vertical)
    }

    fn padding_start(self, start: f32) -> Padding {
        self.padding(start, 0.0, 0.0, 0.0)
    }

    fn padding_end(self, end: f32) -> Padding {
        self.padding(0.0, end, 0.0, 0.0)
    }

    fn padding_top(self, top: f32) -> Padding {
        self.padding(0.0, 0.0, top, 0.0)
    }

    fn padding_bottom(self, bottom: f32) -> Padding {
        self.padding(0.0, 0.0, 0.0, bottom)
    }
}

impl<V: View + Sized> Paddable for V {}

impl View for Padding {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![self.view.build()]
    }

    fn size(&self, constraints: Constraints, children: Vec<ViewSize>) -> Vec2 {
        if let Some(child) = children.into_iter().next() {
            child.size(constraints) + Vec2::new(self.start + self.end, self.top + self.bottom)
        } else {
            panic!("Padding must have one child view")
        }
    }

    fn layout(&self, size: Vec2, children: Vec<ViewLayout>) {
        if let Some(child) = children.into_iter().next() {
            child.layout(
                Vec2::new(self.start, self.top),
                size - Vec2::new(self.start + self.end, self.top + self.bottom),
            );
        }
    }
}
