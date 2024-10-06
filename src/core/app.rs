use super::{tree::Tree, Shape};
use crate::{utils::id_vec::Id, View};
use macroquad::{
    math::Vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
};
use std::{any::Any, collections::HashMap, rc::Rc};

pub struct App {
    tree: Tree,
    states: HashMap<Id, Rc<dyn Any>>,
}

impl App {
    pub fn new<V: View>(root: V) -> Self {
        App {
            tree: Tree::new(root),
            states: HashMap::new(),
        }
    }

    pub fn update(&mut self, id: Id) {
        self.tree.update(&mut self.states, id);
    }

    pub fn draw(&self) {
        self.tree.traverse_down(|_, node| {
            for shape in node.graphics.iter() {
                match shape {
                    Shape::Rect {
                        position,
                        size,
                        fill,
                        stroke,
                    } => {
                        if let Some(color) = fill {
                            draw_rectangle(position.x, position.y, size.x, size.y, *color)
                        }
                        if let Some((width, color)) = stroke {
                            draw_rectangle_lines(
                                position.x, position.y, size.x, size.y, *width, *color,
                            )
                        }
                    }
                }
            }
            false
        });
    }

    pub fn interact(&self, point: Vec2) {
        self.tree.traverse_up(|_, node| {
            if node.layout.contains(point) {
                node.view.interact()
            } else {
                false
            }
        });
    }
}
