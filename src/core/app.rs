use macroquad::{color::RED, math::Vec2, shapes::draw_rectangle_lines};

use super::{view_node::ViewNode, Context, Layout};
use crate::View;
use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

pub struct App {
    root: ViewNode,
    states: HashMap<usize, Rc<dyn Any>>,
}

impl App {
    pub fn new(root: impl View) -> Self {
        App {
            root: ViewNode::new(root.get_key().id(0, 0), Box::new(root)),
            states: HashMap::new(),
        }
    }

    pub fn build(&mut self) {
        self.build_tree();
        self.calculate_constraints();
        self.calculate_layouts();
    }

    pub fn draw(&self) {
        App::draw_node(&self.root);
    }

    fn draw_node(node: &ViewNode) {
        draw_rectangle_lines(
            node.layout.position.x,
            node.layout.position.y,
            node.layout.size.x,
            node.layout.size.y,
            2.0,
            RED,
        );

        for child in node.children.iter() {
            App::draw_node(child);
        }
    }

    pub fn interact(&self, point: Vec2) {
        App::interact_node(&self.root, point);
    }

    fn interact_node(node: &ViewNode, point: Vec2) {
        if node.layout.contains(point) {
            node.view.interact();
        }

        for child in node.children.iter() {
            App::interact_node(child, point);
        }
    }

    pub fn print(&self) {
        App::print_node(&self.root, "".into());
    }

    fn print_node(node: &ViewNode, indent: String) {
        println!(
            "{}({}): {}",
            node.view.get_debug_string(),
            node.id,
            size_of_val(&*node.view)
        );
        for (index, child) in node.children.iter().enumerate() {
            let last = index == node.children.len() - 1;
            print!("{}{} ", indent, if last { "╚" } else { "╠" });
            App::print_node(
                child,
                if last {
                    indent.clone() + "  ".into()
                } else {
                    indent.clone() + "║ ".into()
                },
            );
        }
    }
}

impl App {
    fn build_tree(&mut self) {
        App::build_node(&mut self.root, &mut self.states);
    }

    fn build_node(node: &mut ViewNode, states: &mut HashMap<usize, Rc<dyn Any>>) {
        node.children = node
            .view
            .get_children(&mut Context::new(0, states))
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let id = v.get_key().id(node.id, i);
                ViewNode::new(id, v.clone())
            })
            .collect::<Box<_>>();
        for child in node.children.iter_mut() {
            App::build_node(child, states);
        }
    }
}

impl App {
    fn calculate_constraints(&mut self) {
        App::calculate_node_constraints(&mut self.root);
    }

    fn calculate_node_constraints(node: &mut ViewNode) {
        let child_constraints = node
            .children
            .iter_mut()
            .map(|child| {
                App::calculate_node_constraints(child);
                child.constraints
            })
            .collect::<Box<[_]>>();

        node.constraints = node.view.get_constraints(&child_constraints);
    }
}

impl App {
    fn calculate_layouts(&mut self) {
        let layout = Layout {
            position: Vec2::new(0.0, 0.0),
            size: self.root.constraints.size,
        };

        App::calculate_node_layouts(&mut self.root, layout);
    }

    fn calculate_node_layouts(node: &mut ViewNode, layout: Layout) {
        let child_constraints = node
            .children
            .iter_mut()
            .map(|child| child.constraints)
            .collect::<Box<[_]>>();

        node.layout = layout;
        node.view
            .get_children_layouts(layout, &child_constraints)
            .iter()
            .zip(node.children.iter_mut())
            .for_each(|(layout, child)| {
                App::calculate_node_layouts(child, *layout);
            });
    }
}
