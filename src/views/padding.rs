use std::rc::Rc;

use crate::{
    core::{Constraints, Context, Layout, ViewBuilder},
    View,
};
use macroquad::math::Vec2;

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
    fn get_children(&self, _ctx: &mut Context) -> Box<[Rc<dyn View>]> {
        let view = self.view.build();
        Box::new([view])
    }

    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        Constraints {
            size: child_constraints[0].size
                + Vec2::new(self.start + self.end, self.top + self.bottom),
        }
    }

    fn get_children_layouts(
        &self,
        layout: Layout,
        _child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
        Box::new([Layout {
            position: layout.position + Vec2::new(self.start, self.top),
            size: layout.size - Vec2::new(self.start + self.end, self.top + self.bottom),
        }])
    }
}
