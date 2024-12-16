use crate::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Interaction {
    MouseMove(Vec2),
    MouseDown(Vec2),
    MouseUp(Vec2),
}

impl Interaction {
    pub fn translate_into(self, translation: Vec2) -> Self {
        match self {
            Self::MouseMove(position) => Self::MouseMove(position - translation),
            Self::MouseDown(position) => Self::MouseDown(position - translation),
            Self::MouseUp(position) => Self::MouseUp(position - translation),
        }
    }
}
