use crate::core::{Constraints, Context, View, ViewLayout, ViewSize};
use macroquad::math::Vec2;
use std::rc::Rc;

pub trait Component: 'static + PartialEq {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn build(&self, ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![Rc::new(self.build(ctx))]
    }

    fn size(&self, constraints: Constraints, children: &[ViewSize]) -> Vec2 {
        children[0].size(constraints)
    }

    fn layout(&self, size: Vec2, children: &[ViewLayout]) {
        children[0].layout(Vec2::ZERO, size);
    }
}
