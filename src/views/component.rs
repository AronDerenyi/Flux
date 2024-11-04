use crate::core::{Child, Constraints, Context, View};
use macroquad::math::Vec2;
use std::rc::Rc;

pub trait Component: 'static + PartialEq {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn build(&self, ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![Rc::new(self.build(ctx))]
    }

    fn size(&self, constraints: Constraints, children: &Vec<Child>) -> Vec2 {
        if let Some(child) = children.into_iter().next() {
            child.size(constraints)
        } else {
            panic!("Component must have one child view")
        }
    }

    fn layout(&self, size: Vec2, children: Vec<Child>) {
        if let Some(child) = children.into_iter().next() {
            child.layout(Vec2::ZERO, size);
        }
    }
}
