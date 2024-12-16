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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MouseState {
    Idle,
    Hover,
    Pressed,
}

pub struct MouseListener {
    action: Rc<dyn Fn(&mut ContextMut, MouseState, MouseState) + 'static>,
    view: ViewBuilder,
}

impl PartialEq for MouseListener {
    fn eq(&self, other: &Self) -> bool {
        self.view == other.view && Rc::ptr_eq(&self.action, &other.action)
    }
}

pub trait MouseListenerExt: View + Sized {
    fn on_mouse<A: Fn(&mut ContextMut, MouseState, MouseState) + 'static>(
        self,
        action: A,
    ) -> MouseListener {
        MouseListener {
            action: Rc::new(action),
            view: ViewBuilder::from_view(self),
        }
    }

    fn on_click<A: Fn(&mut ContextMut) + 'static>(self, action: A) -> MouseListener {
        self.on_mouse(move |ctx, prev_state, state| {
            if prev_state == MouseState::Pressed && state == MouseState::Hover {
                action(ctx)
            }
        })
    }
}

impl<V: View + Sized> MouseListenerExt for V {}

impl View for MouseListener {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        let action = self.action.clone();
        let state = context.state(|| MouseState::Idle);

        vec![
            self.view.build(),
            Rc::new(InteractionHandler {
                action: move |ctx, size, interaction| match (*ctx.get(state), interaction) {
                    (MouseState::Idle, Interaction::MouseMove(point)) => {
                        if inside(size, point) {
                            *ctx.get_mut(state) = MouseState::Hover;
                            action(ctx, MouseState::Idle, MouseState::Hover);
                        }
                        false
                    }
                    (MouseState::Idle, Interaction::MouseDown(point)) => {
                        if inside(size, point) {
                            *ctx.get_mut(state) = MouseState::Pressed;
                            action(ctx, MouseState::Idle, MouseState::Pressed);
                            true
                        } else {
                            false
                        }
                    }
                    (MouseState::Hover, Interaction::MouseMove(point)) => {
                        if !inside(size, point) {
                            *ctx.get_mut(state) = MouseState::Idle;
                            action(ctx, MouseState::Hover, MouseState::Idle);
                        }
                        false
                    }
                    (MouseState::Hover, Interaction::MouseDown(point)) => {
                        if inside(size, point) {
                            *ctx.get_mut(state) = MouseState::Pressed;
                            action(ctx, MouseState::Hover, MouseState::Pressed);
                            true
                        } else {
                            false
                        }
                    }
                    (MouseState::Pressed, Interaction::MouseUp(point)) => {
                        if inside(size, point) {
                            *ctx.get_mut(state) = MouseState::Hover;
                            action(ctx, MouseState::Pressed, MouseState::Hover);
                            true
                        } else {
                            *ctx.get_mut(state) = MouseState::Idle;
                            action(ctx, MouseState::Pressed, MouseState::Idle);
                            true
                        }
                    }
                    _ => false,
                },
            }),
        ]
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        children[0].size(constraints)
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        vec![
            Layout {
                position: Vec2::ZERO,
                size: layout.size,
            },
            Layout {
                position: Vec2::ZERO,
                size: layout.size,
            },
        ]
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
        let interaction = interaction.translate_into(layout.position);

        if children[0].interact(context, interaction) {
            true
        } else if children[1].interact(context, interaction) {
            true
        } else {
            false
        }

        // if let Interaction::MouseDown(point) = interaction {
        //     if point.x >= 0.0
        //         && point.y >= 0.0
        //         && point.x <= layout.size.x
        //         && point.y <= layout.size.y
        //     {
        //         (self.action)(context);
        //         true
        //     } else {
        //         false
        //     }
        // } else {
        //     false
        // }
    }
}

pub struct InteractionHandler<A: Fn(&mut ContextMut, Vec2, Interaction) -> bool + 'static> {
    action: A,
}

impl<A: Fn(&mut ContextMut, Vec2, Interaction) -> bool + 'static> PartialEq
    for InteractionHandler<A>
{
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl<A: Fn(&mut ContextMut, Vec2, Interaction) -> bool + 'static> View for InteractionHandler<A> {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        Vec::new()
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        Vec2::new(
            match constraints.width {
                Constraint::Ideal => 0.0,
                Constraint::Min => 0.0,
                Constraint::Max => f32::INFINITY,
                Constraint::Fixed(width) => width,
            },
            match constraints.height {
                Constraint::Ideal => 0.0,
                Constraint::Min => 0.0,
                Constraint::Max => f32::INFINITY,
                Constraint::Fixed(height) => height,
            },
        )
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        Vec::new()
    }

    fn draw(&self, layout: Layout, painter: &mut Painter, children: &[ViewDrawer]) {}

    fn interact(
        &self,
        context: &mut ContextMut,
        layout: Layout,
        interaction: Interaction,
        children: &[ViewInteractor],
    ) -> bool {
        (self.action)(context, layout.size, interaction)
    }
}

fn inside(size: Vec2, point: Vec2) -> bool {
    point.x >= 0.0 && point.y >= 0.0 && point.x <= size.x && point.y <= size.y
}
