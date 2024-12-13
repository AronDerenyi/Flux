use crate::math::Vec2;
use crate::{
    core::{
        Constraint, Constraints, Context, ContextMut, Interaction, Layout, View, ViewDrawer,
        ViewInteractor, ViewSizer,
    },
    graphics::Painter,
};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Spacer {
    ideal: Vec2,
    min: Vec2,
    max: Vec2,
}

pub fn spacer() -> Spacer {
    Spacer {
        ideal: Vec2::ZERO,
        min: Vec2::ZERO,
        max: Vec2::INFINITY,
    }
}

impl Spacer {
    pub fn width(mut self, width: f32) -> Self {
        self.ideal.x = width;
        self.min.x = width;
        self.max.x = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.ideal.y = height;
        self.min.y = height;
        self.max.y = height;
        self
    }

    pub fn ideal_width(mut self, ideal_width: f32) -> Self {
        self.ideal.x = ideal_width;
        self.min.x = self.min.x.min(ideal_width);
        self.max.x = self.max.x.max(ideal_width);
        self
    }

    pub fn ideal_height(mut self, ideal_height: f32) -> Self {
        self.ideal.y = ideal_height;
        self.min.y = self.min.y.min(ideal_height);
        self.max.y = self.max.y.max(ideal_height);
        self
    }

    pub fn min_width(mut self, min_width: f32) -> Self {
        self.ideal.x = self.ideal.x.max(min_width);
        self.min.x = min_width;
        self.max.x = self.max.x.max(min_width);
        self
    }

    pub fn min_height(mut self, min_height: f32) -> Self {
        self.ideal.y = self.ideal.y.max(min_height);
        self.min.y = min_height;
        self.max.y = self.max.y.max(min_height);
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.ideal.x = self.ideal.x.min(max_width);
        self.min.x = self.min.x.min(max_width);
        self.max.x = max_width;
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.ideal.y = self.ideal.y.min(max_height);
        self.min.y = self.min.y.min(max_height);
        self.max.y = max_height;
        self
    }
}

impl View for Spacer {
    fn build(&self, context: &mut Context) -> Vec<Rc<dyn View>> {
        Vec::new()
    }

    fn size(&self, constraints: Constraints, children: &[ViewSizer]) -> Vec2 {
        Vec2::new(
            match constraints.width {
                Constraint::Ideal => self.ideal.x,
                Constraint::Min => self.min.x,
                Constraint::Max => self.max.x,
                Constraint::Fixed(width) => width.clamp(self.min.x, self.max.x),
            },
            match constraints.height {
                Constraint::Ideal => self.ideal.y,
                Constraint::Min => self.min.y,
                Constraint::Max => self.max.y,
                Constraint::Fixed(height) => height.clamp(self.min.y, self.max.y),
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
        false
    }
}
