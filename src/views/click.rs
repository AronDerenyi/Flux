use crate::{
    view::{Constraints, Layout, ViewBuilder},
    view_tree::Context,
    View,
};

#[derive(Clone)]
pub struct Click<V: View + Clone, A: Fn() + Clone> {
    action: A,
    view: ViewBuilder<V>,
}

pub trait Clickable: View + Clone {
    fn on_click<A: Fn() + Clone + 'static>(self, action: A) -> Click<Self, A> {
        Click {
            action,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Clone> Clickable for V {}

impl<V: View + Clone, A: Fn() + Clone + 'static> View for Click<V, A> {
    fn get_children(&self, _ctx: &mut Context) -> Box<[Box<dyn View>]> {
        let view = self.view.build();
        Box::new([Box::new(view)])
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
