use crate::{
    core::{Constraints, ContentBuilder, Context, Layout},
    View,
};
use macroquad::math::Vec2;

#[derive(Clone)]
pub struct Column {
    spacing: f32,
    content: ContentBuilder,
}

impl Column {
    pub fn new(content: ContentBuilder) -> Self {
        Self {
            spacing: 0.0,
            content,
        }
    }

    pub fn spacing(self, spacing: f32) -> Self {
        Self { spacing, ..self }
    }
}

impl View for Column {
    fn get_children(&self, _ctx: &mut Context) -> Box<[Box<dyn View>]> {
        self.content.build()
    }

    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
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

    fn get_children_layouts(
        &self,
        layout: Layout,
        child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
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
        layouts.into()
    }
}
