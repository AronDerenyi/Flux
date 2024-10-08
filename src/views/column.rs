use crate::{
    core::{Constraints, ContentBuilder, Context, Layout},
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

    fn calculate_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        let mut constraints = Constraints {
            size: Vec2::default(),
        };
        for child_constraint in child_constraints {
            constraints.size = Vec2::new(
                constraints.size.x.max(child_constraint.size.x),
                constraints.size.y + child_constraint.size.y,
            );
        }
        constraints.size.y += self.spacing * (child_constraints.len() as f32 - 1.0).max(0.0);
        constraints
    }

    fn calculate_layouts(&self, layout: Layout, child_constraints: &[Constraints]) -> Vec<Layout> {
        let x = layout.position.x;
        let mut y = layout.position.y;
        let mut layouts = Vec::new();
        for child_constraint in child_constraints.iter() {
            let layout = Layout {
                position: Vec2::new(x, y),
                size: child_constraint.size,
            };
            layouts.push(layout);
            y += child_constraint.size.y + self.spacing;
        }
        layouts
    }
}
