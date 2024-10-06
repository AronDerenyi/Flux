use crate::{
    core::{Constraints, ContentBuilder, Context, Layout},
    View,
};
use macroquad::math::Vec2;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Row {
    spacing: f32,
    content: ContentBuilder,
}

#[macro_export]
macro_rules! row {
    [$($content:tt)+] => {
        views::row(content![$($content)+])
    };
}

pub fn row(content: ContentBuilder) -> Row {
    Row {
        spacing: 0.0,
        content,
    }
}

impl Row {
    pub fn spacing(self, spacing: f32) -> Self {
        Self { spacing, ..self }
    }
}

impl View for Row {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        self.content.build()
    }

    fn calculate_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        let mut constraints = Constraints {
            size: Vec2::default(),
        };
        for child_constraint in child_constraints {
            constraints.size = Vec2::new(
                constraints.size.x + child_constraint.size.x,
                constraints.size.y.max(child_constraint.size.y),
            );
        }
        constraints.size.x += self.spacing * (child_constraints.len() as f32 - 1.0).max(0.0);
        constraints
    }

    fn calculate_layouts(&self, layout: Layout, child_constraints: &[Constraints]) -> Vec<Layout> {
        let mut x = layout.position.x;
        let y = layout.position.y;
        let mut layouts = Vec::new();
        for child_constraint in child_constraints.iter() {
            let layout = Layout {
                position: Vec2::new(x, y),
                size: child_constraint.size,
            };
            layouts.push(layout);
            x += child_constraint.size.x + self.spacing;
        }
        layouts
    }
}
