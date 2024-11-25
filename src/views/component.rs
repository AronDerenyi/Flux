use crate::core::{
    Constraints, Context, Interaction, Layout, Painter, View, ViewDrawer, ViewInteractor, ViewSizer,
};
use macroquad::math::Vec2;
use std::rc::Rc;

pub trait Component: 'static + PartialEq {
    fn build(&self, ctx: &mut Context) -> impl View;
}

impl<V: Component> View for V {
    fn build(&self, ctx: &mut Context) -> Vec<Rc<dyn View>> {
        vec![Rc::new(self.build(ctx))]
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
        layout: Layout,
        interaction: Interaction,
        children: &[ViewInteractor],
    ) -> bool {
        children[0].interact(interaction.translate_into(layout.position))
    }
}
