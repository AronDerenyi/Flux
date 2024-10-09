use crate::{
    core::{Constraints, ContentBuilder, Context, Layout, Position, Size, SizeHint},
    View,
};
use macroquad::math::Vec2;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Column {
    spacing: f32,
    content: ContentBuilder,
}

#[macro_export]
macro_rules! column {
    [$($content:tt)+] => {
        views::column(content![$($content)+])
    };
}

pub fn column(content: ContentBuilder) -> Column {
    Column {
        spacing: 0.0,
        content,
    }
}

impl Column {
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl View for Column {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        self.content.build()
    }

    // fn calculate_layouts(&self, layout: Layout, child_constraints: &[Constraints]) -> Vec<Layout> {
    //     let x = layout.position.x;
    //     let mut y = layout.position.y;
    //     let mut layouts = Vec::new();
    //     for child_constraint in child_constraints.iter() {
    //         let layout = Layout {
    //             position: Vec2::new(x, y),
    //             size: child_constraint.size,
    //         };
    //         layouts.push(layout);
    //         y += child_constraint.size.y + self.spacing;
    //     }
    //     layouts
    // }

    fn calculate_size_hint(&self, child_size_hints: &[SizeHint]) -> SizeHint {
        let mut size_hint = SizeHint::default();
        for child_size_hint in child_size_hints {
            size_hint.min_width = size_hint.min_width.max(child_size_hint.min_width);
            size_hint.min_height = child_size_hint.min_height;
            size_hint.ideal_width = size_hint.min_width.max(child_size_hint.ideal_width);
            size_hint.ideal_height = child_size_hint.ideal_height;
            size_hint.max_width = size_hint.min_width.max(child_size_hint.max_width);
            size_hint.max_height = child_size_hint.max_height;
        }
        let spacing = self.spacing * (child_size_hints.len() as f32 - 1.0).max(0.0);
        size_hint.min_width += spacing;
        size_hint.ideal_width += spacing;
        size_hint.max_width += spacing;
        size_hint
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
