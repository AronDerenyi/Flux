pub mod color;
pub mod paint;
pub mod painter;
pub mod paragraph;
pub(crate) mod renderer;

pub mod prelude {
    pub use super::{
        color::Color,
        paint::Paint,
        painter::Painter,
        paragraph::{Paragraph, ParagraphStyle},
    };
}
