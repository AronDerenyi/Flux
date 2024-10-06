use crate::{
    core::{Constraints, Context, Layout, ViewBuilder},
    View,
};
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

    fn calculate_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        child_constraints[0]
    }

    fn calculate_layouts(&self, layout: Layout, _child_constraints: &[Constraints]) -> Vec<Layout> {
        vec![layout]
    }

    fn interact(&self) -> bool {
        (self.action)();
        true
    }
}
