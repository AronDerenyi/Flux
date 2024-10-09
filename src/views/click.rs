use crate::{
    core::{Constraints, Context, Position, Size, SizeHint, ViewBuilder},
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

    fn calculate_size_hint(&self, child_size_hints: &[SizeHint]) -> SizeHint {
        child_size_hints[0]
    }

    fn calculate_constraints(
        &self,
        constraints: Constraints,
        child_size_hints: &[SizeHint],
    ) -> Vec<Constraints> {
        vec![constraints]
    }

    fn calculate_layout(
        &self,
        constraints: Constraints,
        child_sizes: &[Size],
    ) -> (Size, Vec<Position>) {
        (child_sizes[0], vec![Position::default()])
    }

    fn interact(&self) -> bool {
        (self.action)();
        true
    }
}
