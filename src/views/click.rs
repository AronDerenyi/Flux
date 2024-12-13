use super::ViewBuilder;
use crate::math::Vec2;
use crate::{
    core::{
        Constraints, Context, ContextMut, Interaction, Layout, View, ViewDrawer, ViewInteractor,
        ViewSizer,
    },
    graphics::Painter,
};
use std::rc::Rc;

pub struct Click<A: Fn(&mut ContextMut)> {
    action: A,
    view: ViewBuilder,
}

impl<A: Fn(&mut ContextMut)> PartialEq for Click<A> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

pub trait Clickable: View + Sized {
    fn on_click<A: Fn(&mut ContextMut) + 'static>(self, action: A) -> Click<A> {
        Click {
            action,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Sized> Clickable for V {}

impl<A: Fn(&mut ContextMut) + 'static> View for Click<A> {
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
        let Interaction::Click(point) = interaction;
        if children[0].interact(context, interaction.translate_into(layout.position)) {
            true
        } else if point.x >= layout.position.x
            && point.y >= layout.position.y
            && point.x <= layout.position.x + layout.size.x
            && point.y <= layout.position.y + layout.size.y
        {
            (self.action)(context);
            true
        } else {
            false
        }
    }
}
