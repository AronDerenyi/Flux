#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Self::from_hex(0x000000);
    pub const WHITE: Self = Self::from_hex(0xFFFFFF);
    pub const RED: Self = Self::from_hex(0xFF0000);
    pub const GREEN: Self = Self::from_hex(0x00FF00);
    pub const BLUE: Self = Self::from_hex(0x0000FF);
    pub const TRANSPARENT: Self = Self::from_rgba(0, 0, 0, 0);

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(rgba: (u8, u8, u8, u8)) -> Self {
        Self::from_rgba(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgba: (u8, u8, u8)) -> Self {
        Self::from_rgb(rgba.0, rgba.1, rgba.2)
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Self::from_hex(hex)
    }
}
