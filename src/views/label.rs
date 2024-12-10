use std::rc::Rc;

use crate::core::{
    Context, Constraints, ContextMut, Interaction, Layout, Painter, View, ViewDrawer,
    ViewInteractor, ViewSizer,
};
use macroquad::{
    color::{Color, BLACK},
    math::Vec2,
    text::{draw_multiline_text, measure_text},
};

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
        color: BLACK,
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
        Vec::new()
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        let measurements = measure_text(&self.text, None, (self.size * 2.0) as u16, 1.0);
        Vec2::new(measurements.width, self.size * 2.0)
    }

    fn layout(&self, layout: Layout, children: &[ViewSizer]) -> Vec<Layout> {
        Vec::new()
    }

    fn draw(&self, layout: Layout, painter: &mut Painter, children: &[ViewDrawer]) {
        painter.text(
            &self.text,
            layout.position + Vec2::new(0.0, 0.0 + self.size * 1.5),
            self.size,
            self.color,
        );
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
