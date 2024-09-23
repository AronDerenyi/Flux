use crate::core::{Constraints, Context, Layout, View};

pub trait Component: 'static + Clone {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn get_children(&self, ctx: &mut Context) -> Box<[Box<dyn View>]> {
        let child = self.build(ctx);
        Box::new([Box::new(child)])
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
