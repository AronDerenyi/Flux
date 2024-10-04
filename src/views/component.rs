use std::rc::Rc;

use crate::core::{Constraints, Context, Layout, View};

pub trait Component: 'static + PartialEq {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn get_children(&self, ctx: &mut Context) -> Box<[Rc<dyn View>]> {
        let child = self.build(ctx);
        Box::new([Rc::new(child)])
    }
    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        child_constraints[0]
    }
    fn get_children_layouts(
        &self,
        layout: Layout,
        _child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
        Box::new([layout])
    }
}
