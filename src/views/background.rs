use super::ViewBuilder;
use crate::{
    core::{
        Constraints, Context, ContextMut, Interaction, Layout, View, ViewDrawer, ViewInteractor,
        ViewSizer,
    },
    graphics::{Color, Painter},
};
use glam::Vec2;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Background {
    color: Color,
    view: ViewBuilder,
}

pub trait Backgroundable: View + Sized {
    fn background(self, color: impl Into<Color>) -> Background {
        Background {
            color: color.into(),
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Backgroundable for V {}

impl View for Background {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        vec![self.view.build()]
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
            painter.draw_rect(Vec2::ZERO, layout.size, self.color);
            children[0].draw(painter);
        });
    }

    fn interact(
        &self,
        context: &mut ContextMut,
        layout: Layout,
        interaction: Interaction,
        children: &[ViewInteractor],
    ) -> bool {
        children[0].interact(context, interaction.translate_into(layout.position))
    }
}
