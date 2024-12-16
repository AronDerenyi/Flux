pub mod color;
pub mod paint;
pub mod painter;
pub(crate) mod renderer;
pub mod text;

pub mod prelude {
    pub use super::{
        color::Color,
        paint::Paint,
        painter::Painter,
        text::{Text, TextStyle},
    };
}
