use super::tree::Tree;
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
        self.tree.traverse_down(|origin, _, node| {
            node.graphics.draw(Vec2::new(origin.x, origin.y));
            false
        });
    }

    pub fn interact(&self, point: Vec2) {
        self.tree.traverse_up(|origin, _, node| {
            if origin.x < point.x
                && origin.y < point.y
                && origin.x + node.size.width > point.x
                && origin.y + node.size.height > point.y
            {
                node.view.interact()
            } else {
                false
            }
        });
    }
}
