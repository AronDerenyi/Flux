use super::ContentBuilder;
use crate::{
    core::{Constraint, Constraints, Context, ViewLayout, ViewSize},
    View,
};
use macroquad::math::Vec2;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Flex {
    axis: usize,
    spacing: f32,
    content: ContentBuilder,
}

#[macro_export]
macro_rules! row {
    [$($content:tt)+] => {
        $crate::views::row(content![$($content)+])
    };
}

#[macro_export]
macro_rules! column {
    [$($content:tt)+] => {
        $crate::views::column(content![$($content)+])
    };
}

pub fn row(content: ContentBuilder) -> Flex {
    Flex {
        axis: 0,
        spacing: 0.0,
        content,
    }
}

pub fn column(content: ContentBuilder) -> Flex {
    Flex {
        axis: 1,
        spacing: 0.0,
        content,
    }
}

impl Flex {
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl View for Flex {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        self.content.build()
    }

    fn size(&self, constraints: Constraints, children: Vec<ViewSize>) -> Vec2 {
        let main_axis = self.axis;
        let cross_axis = 1 - main_axis;

        let mut size = Vec2::ZERO;
        size[main_axis] = self.spacing * (children.len() as f32 - 1.0).max(0.0);
        for child in children {
            let child_size = child.size(Constraints {
                width: Constraint::Ideal,
                height: Constraint::Ideal,
            });
            size[main_axis] += child_size[main_axis];
            size[cross_axis] = size[cross_axis].max(child_size[cross_axis]);
        }
        size
    }

    fn layout(&self, size: Vec2, children: Vec<ViewLayout>) {
        let main_axis = self.axis;
        let cross_axis = 1 - main_axis;

        let mut offset = Vec2::ZERO;
        for child in children {
            let size = child.size(Constraints {
                width: Constraint::Ideal,
                height: Constraint::Ideal,
            });
            child.layout(offset, size);
            offset[main_axis] += size[main_axis] + self.spacing;
        }
    }
}
