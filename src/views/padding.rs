use crate::{
    core::{Constraints, Context, Position, Size, SizeHint, ViewBuilder},
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

    fn calculate_size_hint(&self, child_size_hints: &[SizeHint]) -> SizeHint {
        let horizontal_padding = self.start + self.end;
        let vertical_padding = self.top + self.bottom;
        SizeHint {
            min_width: child_size_hints[0].min_width + horizontal_padding,
            min_height: child_size_hints[0].min_height + vertical_padding,
            ideal_width: child_size_hints[0].ideal_width + horizontal_padding,
            ideal_height: child_size_hints[0].ideal_height + vertical_padding,
            max_width: child_size_hints[0].max_width + horizontal_padding,
            max_height: child_size_hints[0].max_height + vertical_padding,
        }
    }

    fn calculate_constraints(
        &self,
        constraints: Constraints,
        child_size_hints: &[SizeHint],
    ) -> Vec<Constraints> {
        let horizontal_padding = self.start + self.end;
        let vertical_padding = self.top + self.bottom;
        vec![Constraints {
            min_width: constraints.min_width - horizontal_padding,
            min_height: constraints.min_height - vertical_padding,
            max_width: constraints.max_width - horizontal_padding,
            max_height: constraints.max_height - vertical_padding,
        }]
    }

    fn calculate_layout(
        &self,
        constraints: Constraints,
        child_sizes: &[Size],
    ) -> (Size, Vec<Position>) {
        let horizontal_padding = self.start + self.end;
        let vertical_padding = self.top + self.bottom;
        (
            Size {
                width: child_sizes[0].width + horizontal_padding,
                height: child_sizes[0].height + vertical_padding,
            },
            vec![Position {
                x: self.start,
                y: self.top,
            }],
        )
    }
}
