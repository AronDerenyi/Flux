use super::ContentBuilder;
use crate::{
    core::{Constraint, Constraints, Context, ViewLayout, ViewSize},
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
        $crate::views::column(content![$($content)+])
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

    fn size(&self, constraints: Constraints, children: Vec<ViewSize>) -> Vec2 {
        let mut size = Vec2::ZERO;
        size.y = self.spacing * (children.len() as f32 - 1.0).max(0.0);
        for child in children {
            let child_size = child.size(Constraints {
                width: Constraint::Ideal,
                height: Constraint::Ideal,
            });
            size.x = size.x.max(child_size.x);
            size.y += child_size.y;
        }
        size
    }

    fn layout(&self, size: Vec2, children: Vec<ViewLayout>) {
        let mut y = 0.0;
        for child in children {
            let child_size = child.size(Constraints {
                width: Constraint::Ideal,
                height: Constraint::Ideal,
            });
            child.layout(Vec2::new(0.0, y), child_size);
            y += child_size.y + self.spacing;
        }
    }
}
