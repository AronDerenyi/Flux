use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, BitXor},
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Change(u8);

impl Change {
    pub const NONE: Self = Self(0);
    pub const VIEW: Self = Self(0b000001);
    pub const SIZE_HINT: Self = Self(0b000010);
    pub const CHILD_SIZE_HINT: Self = Self(0b000100);
    pub const CONSTRAINTS: Self = Self(0b001000);
    pub const SIZE: Self = Self(0b010000);
    pub const CHILD_SIZE: Self = Self(0b100000);
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
        let changes = [
            Self::VIEW,
            Self::SIZE_HINT,
            Self::CHILD_SIZE_HINT,
            Self::CONSTRAINTS,
            Self::SIZE,
            Self::CHILD_SIZE,
        ]
        .into_iter()
        .filter(|change| self.contains(*change))
        .map::<String, _>(|change| match change {
            Self::VIEW => "view".into(),
            Self::SIZE_HINT => "size_hint".into(),
            Self::CHILD_SIZE_HINT => "child_size_hint".into(),
            Self::CONSTRAINTS => "constraints".into(),
            Self::SIZE => "size".into(),
            Self::CHILD_SIZE => "child_size".into(),
            _ => "unknown".into(),
        })
        .join(", ");

        f.write_str("[")?;
        f.write_str(&changes)?;
        f.write_str("]")
    }
}
