use super::{Constraints, Context, Painter, Position, Size, SizeHint};
use macroquad::math::Vec2;
use std::{any::Any, rc::Rc};

/*
Improved layout calculation:

build (traverse down)
size hint (traverse up) save(min: Vec2, ideal: Vec2, max: Vec2) fn(child size hints) -> size hint
constraints (traverse down) save(min: Vec2, max: Vec2) fn(constraints, child size hints) -> child constraints
size (traverse up) save(size: Vec2) fn(constraints, child sizes) -> size
position (traverse down) save(position: Vec2) fn(position, child sizes) -> child positions
*/

#[allow(unused, private_bounds)]
pub trait View: 'static + ViewEq {
    fn build(&self, ctx: &mut Context) -> Vec<Rc<dyn View>> {
        Default::default()
    }

    fn calculate_size_hint(&self, child_size_hints: &[SizeHint]) -> SizeHint;

    fn calculate_constraints(
        &self,
        constraints: Constraints,
        child_size_hints: &[SizeHint],
    ) -> Vec<Constraints> {
        Default::default()
    }

    fn calculate_layout(
        &self,
        constraints: Constraints,
        child_sizes: &[Size],
    ) -> (Size, Vec<Position>);

    fn draw(&self, size: Size, painter: &mut Painter) {}

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
        ContentBuilder::from_slice([$(std::rc::Rc::new($child)),+])
    );
}
