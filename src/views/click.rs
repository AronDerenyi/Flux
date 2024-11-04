use super::ViewBuilder;
use crate::{
    core::{Child, Constraints, Context},
    View,
};
use macroquad::math::Vec2;
use std::rc::Rc;

pub struct Click<A: Fn()> {
    action: A,
    view: ViewBuilder,
}

impl<A: Fn()> PartialEq for Click<A> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

pub trait Clickable: View + Sized {
    fn on_click<A: Fn() + 'static>(self, action: A) -> Click<A> {
        Click {
            action,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Clickable for V {}

impl<A: Fn() + 'static> View for Click<A> {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![self.view.build()]
    }

    fn size(&self, constraints: Constraints, children: &Vec<Child>) -> Vec2 {
        if let Some(child) = children.into_iter().next() {
            child.size(constraints)
        } else {
            panic!("Click must have one child view")
        }
    }

    fn layout(&self, size: Vec2, children: Vec<Child>) {
        if let Some(child) = children.into_iter().next() {
            child.layout(Vec2::ZERO, size);
        }
    }

    fn interact(&self) -> bool {
        (self.action)();
        true
    }
}
