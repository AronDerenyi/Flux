use super::ViewBuilder;
use crate::math::Vec2;
use crate::{
    core::{
        Constraints, Context, ContextMut, Interaction, Layout, View, ViewDrawer, ViewInteractor,
        ViewSizer,
    },
    graphics::{Color, Paint, Painter},
};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Border {
    width: f32,
    color: Color,
    view: ViewBuilder,
}

pub trait Borderable: View + Sized {
    fn border(self, width: f32, color: impl Into<Color>) -> Border {
        Border {
            width,
            color: color.into(),
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Borderable for V {}

impl View for Border {
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
            painter.draw_rect(
                Vec2::ZERO,
                layout.size,
                Paint::stroke(self.width, self.color),
            );
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
