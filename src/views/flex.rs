use super::ContentBuilder;
use crate::{
    core::{Child, Constraint, Constraints, Context},
    View,
};
use itertools::Itertools;
use macroquad::math::Vec2;
use std::{cmp::Ordering, rc::Rc};

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

    fn size(&self, mut constraints: Constraints, children: &Vec<Child>) -> Vec2 {
        if let Constraint::Fixed(mut available) = constraints[self.axis] {
            constraints[self.axis] = Constraint::Ideal;
            let mut child_sizes = children
                .iter()
                .map(|child| child.size(constraints))
                .collect();
            let size = self.calculate_size(&child_sizes);

            available -= size[self.axis];
            match available.total_cmp(&0.0) {
                Ordering::Equal => return size,
                Ordering::Less => constraints[self.axis] = Constraint::Min,
                Ordering::Greater => constraints[self.axis] = Constraint::Max,
            }
            let mut child_availables = children
                .iter()
                .zip(child_sizes.iter())
                .map(|(child, size)| child.size(constraints)[self.axis] - size[self.axis])
                .collect_vec();

            self.flex_child_sizes(&mut child_sizes, child_availables, available);

            for (child, child_size) in children.iter().zip(child_sizes.iter_mut()) {
                constraints[self.axis] = Constraint::Fixed(child_size[self.axis]);
                *child_size = child.size(constraints);
            }

            self.calculate_size(&child_sizes)
        } else {
            self.calculate_size(
                &children
                    .iter()
                    .map(|child| child.size(constraints))
                    .collect(),
            )
        }
    }

    fn layout(&self, size: Vec2, children: Vec<Child>) {
        let mut constraints = Constraints {
            width: Constraint::Fixed(size.x),
            height: Constraint::Fixed(size.y),
        };

        constraints[self.axis] = Constraint::Ideal;
        let mut child_sizes = children
            .iter()
            .map(|child| child.size(constraints))
            .collect();
        let ideal_size = self.calculate_size(&child_sizes);

        let mut available = size[self.axis] - ideal_size[self.axis];
        if available != 0.0 {
            if available < 0.0 {
                constraints[self.axis] = Constraint::Min;
            } else {
                constraints[self.axis] = Constraint::Max;
            }

            let mut child_availables = children
                .iter()
                .zip(child_sizes.iter())
                .map(|(child, size)| child.size(constraints)[self.axis] - size[self.axis])
                .collect_vec();

            self.flex_child_sizes(&mut child_sizes, child_availables, available);

            for (child, child_size) in children.iter().zip(child_sizes.iter_mut()) {
                constraints[self.axis] = Constraint::Fixed(child_size[self.axis]);
                *child_size = child.size(constraints);
            }
        }

        let mut offset = Vec2::ZERO;
        for (child, child_size) in children.into_iter().zip(child_sizes) {
            child.layout(offset, child_size);
            offset[self.axis] += child_size[self.axis] + self.spacing;
        }
    }
}

impl Flex {
    fn calculate_size(&self, child_sizes: &Vec<Vec2>) -> Vec2 {
        let main_axis = self.axis;
        let cross_axis = 1 - main_axis;

        let mut size = Vec2::ZERO;
        size[main_axis] = self.spacing * (child_sizes.len() as f32 - 1.0).max(0.0);

        for child_size in child_sizes {
            size[main_axis] += child_size[main_axis];
            size[cross_axis] = size[cross_axis].max(child_size[cross_axis]);
        }
        size
    }

    fn flex_child_sizes(
        &self,
        child_sizes: &mut Vec<Vec2>,
        mut child_availables: Vec<f32>,
        mut available: f32,
    ) {
        while available.abs() > 0.01 {
            let flexibles = child_availables
                .iter()
                .filter(|available| available.abs() > 0.01)
                .count();

            if flexibles == 0 {
                break;
            }

            let flex = available / flexibles as f32;
            for (child_size, child_available) in
                child_sizes.iter_mut().zip(child_availables.iter_mut())
            {
                if child_available.abs() > 0.01 {
                    let change = if available > 0.0 {
                        f32::min(*child_available, flex)
                    } else {
                        f32::max(*child_available, flex)
                    };

                    child_size[self.axis] += change;
                    *child_available -= change;
                    available -= change;
                }
            }
        }
    }
}
