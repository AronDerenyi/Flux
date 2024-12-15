pub mod core;
pub mod graphics;
pub mod utils;
pub mod views;

pub mod math {
    pub use glam::*;
    pub mod prelude {
        pub use super::Vec2;
    }
}

pub mod prelude {
    pub use crate::{core::prelude::*, graphics::prelude::*, math::prelude::*, views::prelude::*};
}
