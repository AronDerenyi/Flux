#![allow(unused)]
pub mod click;
pub mod component;
pub mod decoration;
pub mod flex;
pub mod label;
pub mod padding;
pub mod spacer;

pub mod prelude {
    pub use super::{
        click::Clickable,
        component::Component,
        decoration::{Border, BoxDecoration, Decoratable, Decoration},
        flex::{col, row},
        label::label,
        padding::Paddable,
        spacer::spacer,
        ContentBuilder, ViewBuilder,
    };
    pub use crate::{col, content, row};
}

use crate::core::view::View;
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

    pub fn from_items<T, V: View, I: Iterator<Item = T>, F: FnMut(T) -> V>(
        items: I,
        mut builder: F,
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

/*
/// This is an example workaround to make closures implement PartialEq
/// allowing views like buttons to have callbacks and still implement PartialEq.
/// Only drawback is that the captured values must be explicitly moved.

#[derive(PartialEq)]
struct Callback<C, I, O> {
    captured: C,
    function: fn(&C, I) -> O,
}

impl<C, I, O> Callback<C, I, O> {
    fn call(&self, input: I) -> O {
        (self.function)(&self.captured, input);
    }
}

fn create_print_callback<C: Display>(captured: C) -> Callback<C, (), ()> {
    Callback {
        captured,
        function: |captured, _| println!("Captured: {}", captured),
    }
}

fn test() {
    let foo = create_print_callback(42);
    let bar = create_print_callback(42);
    println!("{:?}", foo == bar);
}
*/
