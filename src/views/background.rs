use crate::core::{Constraints, Context, Layout, Shape, View, ViewBuilder};
use macroquad::color::Color;

#[derive(Clone)]
pub struct Background<V: View + Clone> {
    color: Color,
    view: ViewBuilder<V>,
}

pub trait Backgroundable: View + Clone {
    fn background(self, color: Color) -> Background<Self> {
        Background {
            color,
            view: ViewBuilder::from_view(self),
        }
    }
}

impl<V: View + Clone> Backgroundable for V {}

impl<V: View + Clone> View for Background<V> {
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
            fill: Some(self.color),
            stroke: None,
        }])
    }
}
