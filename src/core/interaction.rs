use crate::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Interaction {
    Click(Vec2),
}

impl Interaction {
    pub fn translate_into(self, translation: Vec2) -> Self {
        match self {
            Self::Click(position) => Self::Click(position - translation),
        }
    }
}
