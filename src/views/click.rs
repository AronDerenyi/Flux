use super::ViewBuilder;
use crate::{
    core::{
        Constraints, Context, Interaction, Layout, Painter, ViewDrawer, ViewInteractor, ViewSizer,
    },
    View,
};
use macroquad::math::Vec2;
use std::rc::Rc;

pub struct Click<A: Fn()> {
    action: A,
    view: ViewBuilder,
}

impl<A: Fn()> PartialEq for Click<A> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

pub trait Clickable: View + Sized {
    fn on_click<A: Fn() + 'static>(self, action: A) -> Click<A> {
        Click {
            action,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Clickable for V {}

impl<A: Fn() + 'static> View for Click<A> {
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
            children[0].draw(painter);
        });
    }

    fn interact(
        &self,
        layout: Layout,
        interaction: Interaction,
        children: &[ViewInteractor],
    ) -> bool {
        let Interaction::Click(point) = interaction;
        if children[0].interact(interaction.translate_into(layout.position)) {
            true
        } else if point.x >= layout.position.x
            && point.y >= layout.position.y
            && point.x <= layout.position.x + layout.size.x
            && point.y <= layout.position.y + layout.size.y
        {
            (self.action)();
            true
        } else {
            false
        }
    }
}
