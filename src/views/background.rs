use super::ViewBuilder;
use crate::core::{
    Constraints, Context, Interaction, Layout, Painter, View, ViewDrawer, ViewInteractor, ViewSizer,
};
use macroquad::{color::Color, math::Vec2};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Background {
    color: Color,
    view: ViewBuilder,
}

pub trait Backgroundable: View + Sized {
    fn background(self, color: Color) -> Background {
        Background {
            color,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Backgroundable for V {}

impl View for Background {
    fn build(&self, _ctx: &mut Context) -> Vec<Rc<dyn View>> {
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
            painter.rect_filled(Vec2::ZERO, layout.size, self.color);
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
