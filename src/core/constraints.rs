use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Constraints {
    pub width: Constraint,
    pub height: Constraint,
}

#[derive(Clone, Copy)]
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
            (Constraint::Fixed(a), Constraint::Fixed(b)) => a.to_ne_bytes() == b.to_ne_bytes(),
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
                value.to_ne_bytes().hash(state);
            }
        }
    }
}
