#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SizeHint {
    pub min_width: f32,
    pub min_height: f32,
    pub ideal_width: f32,
    pub ideal_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Constraints {
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
