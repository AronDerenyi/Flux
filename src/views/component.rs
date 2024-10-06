use std::rc::Rc;

use crate::core::{Constraints, Context, Layout, View};

pub trait Component: 'static + PartialEq {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn build(&self, ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![Rc::new(self.build(ctx))]
    }
    fn calculate_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        child_constraints[0]
    }
    fn calculate_layouts(&self, layout: Layout, _child_constraints: &[Constraints]) -> Vec<Layout> {
        vec![layout]
    }
}
