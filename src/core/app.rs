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
        self.tree.traverse(Vec2::ZERO, &|node, origin, children| {
            let origin = origin + node.position;
            node.graphics.draw(origin);
            for child in children {
                child.visit(origin);
            }
        });
    }

    pub fn interact(&self, point: Vec2) {
        self.tree
            .traverse(Vec2::ZERO, &move |node, origin, children| {
                let origin = origin + node.position;
                for child in children {
                    if child.visit(origin) {
                        return true;
                    }
                }

                let relative = point - origin;
                if relative.x > 0.0
                    && relative.y > 0.0
                    && relative.x < node.size.x
                    && relative.y < node.size.y
                {
                    node.view.interact()
                } else {
                    false
                }
            });
    }
}
