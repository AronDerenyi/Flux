#![allow(unused)]
mod background;
mod border;
mod click;
mod column;
mod component;
mod label;
mod padding;
mod row;
mod spacer;

pub use background::Backgroundable;
pub use border::Borderable;
pub use click::Clickable;
pub use column::column;
pub use component::Component;
pub use label::label;
pub use padding::Paddable;
pub use row::row;
pub use spacer::spacer;

use crate::core::View;
use std::rc::Rc;

pub struct ViewBuilder {
    builder: Rc<dyn Fn() -> Rc<dyn View>>,
}

impl PartialEq for ViewBuilder {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.builder, &other.builder)
    }
}

impl ViewBuilder {
    pub fn new<F: Fn() -> Rc<dyn View> + 'static>(builder: F) -> Self {
        Self {
            builder: Rc::new(builder),
        }
    }

    pub fn from_view<V: View>(view: V) -> Self {
        let reference = Rc::new(view);
        Self::new(move || reference.clone())
    }

    pub fn build(&self) -> Rc<dyn View> {
        (self.builder)()
    }
}

pub struct ContentBuilder {
    builder: Rc<dyn Fn() -> Vec<Rc<dyn View>>>,
}

impl PartialEq for ContentBuilder {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.builder, &other.builder)
    }
}

impl ContentBuilder {
    pub fn new<F: Fn() -> Vec<Rc<dyn View>> + 'static>(builder: F) -> Self {
        Self {
            builder: Rc::new(builder),
        }
    }

    pub fn from_slice<const N: usize>(views: [Rc<dyn View>; N]) -> Self {
        Self::new(move || views.clone().into())
    }

    pub fn from_vec(views: Vec<Rc<dyn View>>) -> Self {
        Self::new(move || views.clone())
    }

    pub fn from_items<T, V: View, I: Iterator<Item = T>, F: Fn(T) -> V + 'static>(
        items: I,
        builder: F,
    ) -> Self {
        Self::from_vec(
            items
                .map::<Rc<dyn View>, _>(|item| Rc::new(builder(item)))
                .collect(),
        )
    }

    pub fn build(&self) -> Vec<Rc<dyn View>> {
        (self.builder)()
    }
}

#[macro_export]
macro_rules! content {
    [$($child:expr),+ $(,)?] => (
        $crate::views::ContentBuilder::from_slice([$(std::rc::Rc::new($child)),+])
    );
}
