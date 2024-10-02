use crate::core::{Constraints, Context, Layout, Shape, View, ViewBuilder};
use macroquad::color::Color;

#[derive(Clone)]
pub struct Border<V: View + Clone> {
    width: f32,
    color: Color,
    view: ViewBuilder<V>,
}

pub trait Borderable: View + Clone {
    fn border(self, width: f32, color: Color) -> Border<Self> {
        Border {
            width,
            color,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Clone> Borderable for V {}

impl<V: View + Clone> View for Border<V> {
    fn get_children(&self, _ctx: &mut Context) -> Box<[Box<dyn View>]> {
        let view = self.view.build();
        Box::new([Box::new(view)])
    }

    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints {
        child_constraints[0]
    }

    fn get_children_layouts(
        &self,
        layout: Layout,
        _child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
        [layout].into()
    }

    fn draw(&self, layout: Layout) -> Box<[Shape]> {
        Box::new([Shape::Rect {
            position: layout.position,
            size: layout.size,
            fill: None,
            stroke: Some((self.width, self.color)),
        }])
    }
}
