use std::{
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Constraints {
    pub width: Constraint,
    pub height: Constraint,
}

#[derive(Clone, Copy, Debug)]
pub enum Constraint {
    Ideal,
    Min,
    Max,
    Fixed(f32),
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Constraint::Ideal, Constraint::Ideal) => true,
            (Constraint::Min, Constraint::Min) => true,
            (Constraint::Max, Constraint::Max) => true,
            (Constraint::Fixed(a), Constraint::Fixed(b)) => {
                ((a * 100.0) as i32) == ((b * 100.0) as i32)
            }
            _ => false,
        }
    }
}

impl Eq for Constraint {}

impl Hash for Constraint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Constraint::Ideal => 0.hash(state),
            Constraint::Min => 1.hash(state),
            Constraint::Max => 2.hash(state),
            Constraint::Fixed(value) => {
                3.hash(state);
                ((value * 100.0) as i32).hash(state);
            }
        }
    }
}

impl Index<usize> for Constraints {
    type Output = Constraint;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.width,
            1 => &self.height,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Constraints {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.width,
            1 => &mut self.height,
            _ => panic!("Index out of bounds"),
        }
    }
}
