use crate::core::{Constraints, Context, Position, Size, SizeHint, View};
use std::rc::Rc;

pub trait Component: 'static + PartialEq {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn build(&self, ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![Rc::new(self.build(ctx))]
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
}
