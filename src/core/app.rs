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
        self.draw_node(self.tree.root);
    }

    fn draw_node(&self, id: Id) {
        let node = &self.tree[id];

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
                        draw_rectangle_lines(position.x, position.y, size.x, size.y, *width, *color)
                    }
                }
            }
        }

        for child_id in node.children.iter() {
            self.draw_node(*child_id);
        }
    }

    pub fn interact(&self, point: Vec2) {
        self.interact_node(self.tree.root, point);
    }

    fn interact_node(&self, id: Id, point: Vec2) {
        let node = &self.tree[id];

        for child_id in node.children.iter() {
            self.interact_node(*child_id, point);
        }

        if node.layout.contains(point) {
            node.view.interact();
        }
    }
}
