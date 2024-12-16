use crate::math::Vec2;
use crate::{
    core::{
        constraints::Constraints,
        context::{Context, ContextMut},
        interaction::Interaction,
        layout::Layout,
        view::View,
        view_tree::{ViewDrawer, ViewInteractor, ViewSizer},
    },
    graphics::painter::Painter,
};
use std::rc::Rc;

pub trait Component: 'static + PartialEq {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        vec![Rc::new(self.build(context))]
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        children[0].size(constraints)
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        vec![Layout {
            position: Vec2::ZERO,
            size: layout.size,
        }]
    }

    fn draw(&self, layout: Layout, painter: &mut Painter, children: &[ViewDrawer]) {
        painter.translate(layout.position, |painter| {
            children[0].draw(painter);
        });
    }

    fn interact(
        &self,
        context: &mut ContextMut,
        layout: Layout,
        interaction: Interaction,
        consumed: bool,
        children: &[ViewInteractor],
    ) -> bool {
        children[0].interact(
            context,
            interaction.translate_into(layout.position),
            consumed,
        )
    }
}
