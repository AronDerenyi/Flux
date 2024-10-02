use super::{Constraints, Layout};
use crate::{utils::id_vec::Id, views::Spacer, View};
use itertools::Itertools;
use macroquad::{color::Color, math::Vec2};
use std::{
    any::Any,
    cell::{Cell, RefCell},
    fmt::Debug,
    ops::{BitAnd, BitOr, BitXor},
    rc::Rc,
    u8,
};

pub struct ViewNode {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub change: Change,
    pub view: Rc<dyn View>,
    pub constraints: Constraints,
    pub layout: Option<Layout>,
    pub graphics: Box<[Shape]>,
}

impl ViewNode {
    pub fn new(view: Rc<dyn View>, parent: Option<Id>) -> Self {
        ViewNode {
            parent,
            children: Default::default(),
            change: Change::ALL,
            view,
            constraints: Default::default(),
            layout: Default::default(),
            graphics: Default::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Change(u8);

impl Change {
    pub const NONE: Self = Self(0);
    pub const VIEW: Self = Self(0b001);
    pub const CONSTRAINTS: Self = Self(0b010);
    pub const LAYOUT: Self = Self(0b100);
    pub const ALL: Self = Self(u8::MAX);

    pub fn add(&mut self, change: Self) {
        self.0 |= change.0;
    }

    pub fn contains(&self, change: Self) -> bool {
        self.0 & change.0 != 0
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl BitOr for Change {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Change(self.0 | rhs.0)
    }
}

impl BitAnd for Change {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Change(self.0 & rhs.0)
    }
}

impl BitXor for Change {
    type Output = Change;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Change(self.0 ^ rhs.0)
    }
}

impl Debug for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let changes = [Self::VIEW, Self::CONSTRAINTS, Self::LAYOUT]
            .into_iter()
            .filter(|change| self.contains(*change))
            .map::<String, _>(|change| match change {
                Self::VIEW => "view".into(),
                Self::CONSTRAINTS => "constraints".into(),
                Self::LAYOUT => "layout".into(),
                _ => "unknown".into(),
            })
            .join(", ");

        f.write_str("[")?;
        f.write_str(&changes)?;
        f.write_str("]")
    }
}

pub enum Shape {
    Rect {
        position: Vec2,
        size: Vec2,
        fill: Option<Color>,
        stroke: Option<(f32, Color)>,
    },
}
