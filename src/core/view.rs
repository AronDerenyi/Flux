use super::{view_node::Shape, Context};
use core::hash::Hash;
use macroquad::math::Vec2;
use std::{
    any::{type_name, Any},
    rc::Rc,
};

#[allow(unused, private_bounds)]
pub trait View: 'static + ViewEq {
    fn get_children(&self, ctx: &mut Context) -> Box<[Rc<dyn View>]> {
        Default::default()
    }

    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints;

    fn get_children_layouts(
        &self,
        layout: Layout,
        child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
        Default::default()
    }

    fn draw(&self, layout: Layout) -> Box<[Shape]> {
        Default::default()
    }

    fn interact(&self) -> bool {
        false
    }

    fn debug_name(&self) -> &str {
        let mut type_name = std::any::type_name::<Self>();
        if let Some(generic_start) = type_name.find("<") {
            type_name = &type_name[..generic_start];
        }
        if let Some(path_end) = type_name.rfind("::") {
            type_name = &type_name[path_end + 2..];
        }
        type_name
    }
}

trait ViewEq {
    fn as_any(&self) -> &dyn Any;
    fn changed(&self, previous: &dyn Any) -> bool;
}

impl<T: 'static + PartialEq> ViewEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn changed(&self, previous: &dyn Any) -> bool {
        previous
            .downcast_ref::<T>()
            .map_or(false, |previous| self.eq(previous))
    }
}

impl PartialEq for dyn View {
    fn eq(&self, other: &Self) -> bool {
        ViewEq::changed(self, ViewEq::as_any(other))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Constraints {
    pub size: Vec2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Layout {
    pub position: Vec2,
    pub size: Vec2,
}

impl Layout {
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.y >= self.position.y
            && point.x < self.position.x + self.size.x
            && point.y < self.position.y + self.size.y
    }
}

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

#[derive(Clone)]
pub struct ContentBuilder {
    builder: Rc<dyn Fn() -> Box<[Rc<dyn View>]>>,
}

impl PartialEq for ContentBuilder {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.builder, &other.builder)
    }
}

impl ContentBuilder {
    pub fn new<F: Fn() -> Box<[Rc<dyn View>]> + 'static>(builder: F) -> Self {
        Self {
            builder: Rc::new(builder),
        }
    }

    pub fn from_slice<const N: usize>(views: [Rc<dyn View>; N]) -> Self {
        Self::new(move || Box::new(views.clone()))
    }

    pub fn from_boxed_slice(views: Box<[Rc<dyn View>]>) -> Self {
        Self::new(move || views.clone())
    }

    pub fn from_items<T, V: View, I: Iterator<Item = T>, F: Fn(T) -> V + 'static>(
        items: I,
        builder: F,
    ) -> Self {
        Self::from_boxed_slice(
            items
                .map::<Rc<dyn View>, _>(|item| Rc::new(builder(item)))
                .collect(),
        )
    }

    pub fn build(&self) -> Box<[Rc<dyn View>]> {
        (self.builder)()
    }
}

#[macro_export]
macro_rules! content {
    [$($child:expr),+ $(,)?] => (
        ContentBuilder::from_slice([$(std::rc::Rc::new($child)),+])
    );
}
