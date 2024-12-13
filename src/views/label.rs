use crate::{
    core::{
        Constraint, Constraints, Context, ContextMut, Interaction, Layout, View, ViewDrawer,
        ViewInteractor, ViewSizer,
    },
    graphics::{Color, Painter, Paragraph, ParagraphStyle},
};
use glam::Vec2;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Label {
    text: String,
    size: f32,
    color: Color,
}

pub fn label(text: impl Into<String>) -> Label {
    Label {
        text: text.into(),
        size: 12.0,
        color: Color::BLACK,
    }
}

impl Label {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for Label {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        let paragraph = Paragraph::new(
            &self.text,
            ParagraphStyle {
                size: self.size,
                color: self.color,
            },
        );
        vec![Rc::new(ParagraphView { paragraph })]
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
        false
    }
}

struct ParagraphView {
    paragraph: Paragraph,
}

impl PartialEq for ParagraphView {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl View for ParagraphView {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        Vec::new()
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        self.paragraph.size(match constraints.width {
            Constraint::Ideal => f32::INFINITY,
            Constraint::Min => 0.0,
            Constraint::Max => f32::INFINITY,
            Constraint::Fixed(width) => width,
        })
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        Vec::new()
    }

    fn draw(&self, layout: Layout, painter: &mut Painter, children: &[ViewDrawer]) {
        painter.draw_paragraph(&self.paragraph, layout.position, layout.size.x);
    }

    fn interact(
        &self,
        context: &mut ContextMut,
        layout: Layout,
        interaction: Interaction,
        children: &[ViewInteractor],
    ) -> bool {
        todo!()
    }
}
