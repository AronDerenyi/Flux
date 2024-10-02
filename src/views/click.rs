use std::rc::Rc;

use crate::{
    core::{Constraints, Context, Layout, ViewBuilder},
    View,
};

#[derive(Clone)]
pub struct Click<A: Fn()> {
    action: A,
    view: ViewBuilder,
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
    fn get_children(&self, _ctx: &mut Context) -> Box<[Rc<dyn View>]> {
        let view = self.view.build();
        Box::new([view])
    }

    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        child_constraints[0]
    }

    fn get_children_layouts(
        &self,
        layout: Layout,
        _child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
        [layout].into()
    }

    fn interact(&self) -> bool {
        (self.action)();
        true
    }
}
