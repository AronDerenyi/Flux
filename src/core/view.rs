use super::{
    context::Context,
    view_tree::{ViewDrawer, ViewInteractor, ViewSizer, ViewTree},
    Constraints, ContextMut, Interaction, Layout, Painter,
};
use macroquad::math::Vec2;
use std::{any::Any, rc::Rc};

#[allow(unused, private_bounds)]
pub trait View: 'static + ViewEq {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>>;

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2;

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout>;

    fn draw(&self, layout: Layout, painter: &mut Painter, children: &[ViewDrawer]);

    fn interact(
        &self,
        context: &mut ContextMut,
        layout: Layout,
        interaction: Interaction,
        children: &[ViewInteractor],
    ) -> bool;

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
    fn eq(&self, previous: &dyn Any) -> bool;
}

impl<T: 'static + PartialEq> ViewEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<T>()
            .map_or(false, |other| self.eq(other))
    }
}

impl PartialEq for dyn View {
    fn eq(&self, other: &Self) -> bool {
        ViewEq::eq(self, ViewEq::as_any(other))
    }
}
