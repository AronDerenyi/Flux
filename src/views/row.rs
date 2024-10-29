use super::ContentBuilder;
use crate::{
    core::{Constraint, Constraints, Context, ViewLayout, ViewSize},
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
        $crate::views::row(content![$($content)+])
    };
}

pub fn row(content: ContentBuilder) -> Row {
    Row {
        spacing: 0.0,
        content,
    }
}

impl Row {
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl View for Row {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        self.content.build()
    }

    fn size(&self, constraints: Constraints, children: &[ViewSize]) -> Vec2 {
        let mut size = Vec2::ZERO;
        for child in children {
            let child_size = child.size(Constraints {
                width: Constraint::Ideal,
                height: Constraint::Ideal,
            });
            size = Vec2::new(size.x + child_size.x, size.y.max(child_size.y));
        }
        size.x += self.spacing * (children.len() as f32 - 1.0).max(0.0);
        size
    }

    fn layout(&self, size: Vec2, children: &[ViewLayout]) {
        let mut x = 0.0;
        for child in children.iter() {
            let size = child.size(Constraints {
                width: Constraint::Ideal,
                height: Constraint::Ideal,
            });
            child.layout(Vec2::new(x, 0.0), size);
            x += size.x + self.spacing;
        }
    }
}
