use super::{
    tree::{ViewLayout, ViewSize, Visitor},
    Context, Painter,
};
use macroquad::math::Vec2;
use std::{any::Any, rc::Rc};

#[allow(unused, private_bounds)]
pub trait View: 'static + ViewEq {
    fn build(&self, ctx: &mut Context) -> Vec<Rc<dyn View>> {
        Default::default()
    }

    fn size(&self, constraints: Vec2, children: &[ViewSize]) -> Vec2;

    fn layout(&self, size: Vec2, children: &[ViewLayout]) {}

    fn draw(&self, size: Vec2, painter: &mut Painter) {}

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
