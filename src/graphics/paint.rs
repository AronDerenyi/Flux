use super::Color;

pub enum Paint {
    Fill { color: Color },
    Stroke { width: f32, color: Color },
}

impl Paint {
    pub fn fill(color: Color) -> Self {
        Self::Fill { color }
    }

    pub fn stroke(width: f32, color: Color) -> Self {
        Self::Stroke { width, color }
    }
}

impl<T: Into<Color>> From<T> for Paint {
    fn from(value: T) -> Self {
        Self::Fill {
            color: value.into(),
        }
    }
}
