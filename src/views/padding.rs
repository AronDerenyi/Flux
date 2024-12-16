use super::ViewBuilder;
use crate::math::Vec2;
use crate::{
    core::{
        constraints::{Constraint, Constraints},
        context::{Context, ContextMut},
        interaction::Interaction,
        layout::Layout,
        view::View,
        view_tree::{ViewDrawer, ViewInteractor, ViewSizer},
    },
    graphics::painter::Painter,
};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Padding {
    start: f32,
    end: f32,
    top: f32,
    bottom: f32,
    view: ViewBuilder,
}

#[allow(unused)]
pub trait Paddable: View + Sized {
    fn padding(self, start: f32, end: f32, top: f32, bottom: f32) -> Padding {
        Padding {
            start,
            end,
            top,
            bottom,
            view: ViewBuilder::from_view(self),
        }
    }

    fn padding_all(self, padding: f32) -> Padding {
        self.padding(padding, padding, padding, padding)
    }

    fn padding_axial(self, horizontal: f32, vertical: f32) -> Padding {
        self.padding(horizontal, horizontal, vertical, vertical)
    }

    fn padding_horizontal(self, horizontal: f32) -> Padding {
        self.padding(horizontal, horizontal, 0.0, 0.0)
    }

    fn padding_vertical(self, vertical: f32) -> Padding {
        self.padding(0.0, 0.0, vertical, vertical)
    }

    fn padding_start(self, start: f32) -> Padding {
        self.padding(start, 0.0, 0.0, 0.0)
    }

    fn padding_end(self, end: f32) -> Padding {
        self.padding(0.0, end, 0.0, 0.0)
    }

    fn padding_top(self, top: f32) -> Padding {
        self.padding(0.0, 0.0, top, 0.0)
    }

    fn padding_bottom(self, bottom: f32) -> Padding {
        self.padding(0.0, 0.0, 0.0, bottom)
    }
}

impl<V: View + Sized> Paddable for V {}

impl View for Padding {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        vec![self.view.build()]
    }

    fn size(&self, mut constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        if let Constraint::Fixed(width) = constraints.width {
            constraints.width = Constraint::Fixed((width - self.start - self.end).max(0.0));
        }
        if let Constraint::Fixed(height) = constraints.height {
            constraints.height = Constraint::Fixed((height - self.top - self.bottom).max(0.0));
        }
        children[0].size(constraints) + Vec2::new(self.start + self.end, self.top + self.bottom)
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        vec![Layout {
            position: Vec2::new(self.start, self.top),
            size: layout.size - Vec2::new(self.start + self.end, self.top + self.bottom),
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
