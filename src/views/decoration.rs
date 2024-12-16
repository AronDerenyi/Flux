use super::ViewBuilder;
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
    graphics::{color::Color, paint::Paint, painter::Painter},
};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Decorated<D: Decoration> {
    decoration: D,
    background: bool,
    view: ViewBuilder,
}

pub trait Decoratable: View + Sized {
    fn background<D: Decoration>(self, decoration: D) -> Decorated<D> {
        Decorated {
            decoration: decoration.into(),
            background: true,
            view: ViewBuilder::from_view(self),
        }
    }

    fn foreground<D: Decoration>(self, decoration: D) -> Decorated<D> {
        Decorated {
            decoration: decoration.into(),
            background: false,
            view: ViewBuilder::from_view(self),
        }
    }

    fn border(self, width: f32, color: impl Into<Color>) -> Decorated<Border> {
        self.foreground(Border {
            width,
            color: color.into(),
        })
    }
}

impl<V: View + Sized> Decoratable for V {}

impl<D: Decoration> View for Decorated<D> {
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
            if self.background {
                self.decoration.draw(layout.size, painter);
                children[0].draw(painter);
            } else {
                children[0].draw(painter);
                self.decoration.draw(layout.size, painter);
            }
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

pub trait Decoration: PartialEq + 'static {
    fn draw(&self, size: Vec2, painter: &mut Painter);
}

impl Decoration for Color {
    fn draw(&self, size: Vec2, painter: &mut Painter) {
        painter.draw_rect(Vec2::ZERO, size, *self);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Border {
    pub width: f32,
    pub color: Color,
}

impl Decoration for Border {
    fn draw(&self, size: Vec2, painter: &mut Painter) {
        painter.draw_rect(Vec2::ZERO, size, Paint::stroke(self.width, self.color));
    }
}

#[derive(PartialEq)]
pub struct BoxDecoration {
    pub color: Option<Color>,
    pub border: Option<Border>,
    pub radius: f32,
}

impl Decoration for BoxDecoration {
    fn draw(&self, size: Vec2, painter: &mut Painter) {
        if let Some(color) = self.color {
            if self.radius == 0.0 {
                painter.draw_rect(Vec2::ZERO, size, color);
            } else {
                painter.draw_round_rect(Vec2::ZERO, size, self.radius, color);
            }
        }
        if let Some(border) = self.border {
            if self.radius == 0.0 {
                painter.draw_rect(Vec2::ZERO, size, Paint::stroke(border.width, border.color));
            } else {
                painter.draw_round_rect(
                    Vec2::ZERO,
                    size,
                    self.radius,
                    Paint::stroke(border.width, border.color),
                );
            }
        }
    }
}
