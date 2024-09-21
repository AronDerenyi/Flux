use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::Range,
    rc::Rc,
};

use macroquad::math::Vec2;

use crate::{
    view::{Constraints, Layout},
    View,
};

pub struct ViewTree<V: View, R: Fn() -> V> {
    root: R,
    pub nodes: Vec<ViewNode>,
    states: HashMap<usize, Box<dyn Any>>,
}

impl<V: View, R: Fn() -> V> ViewTree<V, R> {
    pub fn new(root: R) -> Self {
        ViewTree {
            root,
            nodes: vec![],
            states: HashMap::new(),
        }
    }

    pub fn build(&mut self) {
        self.nodes = vec![ViewNode {
            view: Box::new((self.root)()),
            children: 0..0,
            constraints: Constraints::default(),
            layout: Layout::default(),
        }];
        self.traverse();
        self.size();
        self.layout();
    }

    fn traverse(&mut self) {
        let mut index = 0;
        while index < self.nodes.len() {
            let children_start = self.nodes.len();
            let mut context = Context {
                id: index,
                states: &mut self.states,
            };
            let children = self.nodes[index].view.get_children(&mut context);
            for child in children {
                self.nodes.push(ViewNode {
                    view: child,
                    children: 0..0,
                    constraints: Constraints::default(),
                    layout: Layout::default(),
                });
            }
            self.nodes[index].children = children_start..self.nodes.len();
            index += 1;
        }
    }

    fn size(&mut self) {
        let mut index = self.nodes.len();
        while index > 0 {
            index -= 1;
            let node = &self.nodes[index];
            let child_constraints = self.nodes[node.children.clone()]
                .iter()
                .map(|child| child.constraints)
                .collect::<Vec<_>>();

            self.nodes[index].constraints = node.view.get_constraints(&child_constraints);
        }
    }

    fn layout(&mut self) {
        let node = &mut self.nodes[0];
        node.layout = Layout {
            position: Vec2::default(),
            size: node.constraints.size,
        };

        let mut index = 0;
        while index < self.nodes.len() {
            let node = &self.nodes[index];
            let children = node.children.clone();
            let child_constraints = self.nodes[children.clone()]
                .iter()
                .map(|child| child.constraints)
                .collect::<Vec<_>>();

            let child_layouts = node
                .view
                .get_children_layouts(node.layout, &child_constraints);

            for (child, layout) in self.nodes[children.clone()].iter_mut().zip(child_layouts) {
                child.layout = layout;
            }

            index += 1;
        }
    }

    pub fn print(&self) {
        let node = &self.nodes[0];
        println!("{}", node.view.get_debug_string());
        self.print_children(node.children.clone(), &"".into());
    }

    fn print_children(&self, children: Range<usize>, indent: &String) {
        for index in children.clone() {
            let last = index == children.end - 1;
            let node = &self.nodes[index];
            println!(
                "{}{} {}: {}",
                indent,
                if last { "╚" } else { "╠" },
                node.view.get_debug_string(),
                size_of_val(&*node.view)
            );
            if !node.children.is_empty() {
                self.print_children(
                    node.children.clone(),
                    &if last {
                        indent.clone() + "  ".into()
                    } else {
                        indent.clone() + "║ ".into()
                    },
                );
            }
        }
    }

    pub fn interaction(&self, point: Vec2) {
        for node in self.nodes.iter().rev() {
            let layout = node.layout;
            if layout.contains(point) {
                if node.view.interact() {
                    break;
                }
            }
        }
    }
}

pub struct ViewNode {
    view: Box<dyn View>,
    children: Range<usize>,
    constraints: Constraints,
    pub layout: Layout,
}

pub struct Context<'a> {
    id: usize,
    states: &'a mut HashMap<usize, Box<dyn Any>>,
}

impl Context<'_> {
    pub fn state<T: Any>(&mut self, init: impl FnOnce() -> T) -> State<T> {
        if let Some(state) = self
            .states
            .get(&self.id)
            .and_then(|s| s.downcast_ref::<State<T>>())
        {
            return state.clone();
        } else {
            let state = State::new(init());
            self.states.insert(self.id, Box::new(state.clone()));
            state
        }
    }
}

pub struct State<T> {
    value: Rc<RefCell<T>>,
}

impl<T> State<T> {
    fn new(value: T) -> Self {
        State {
            value: Rc::new(RefCell::new(value)),
        }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        State {
            value: self.value.clone(),
        }
    }
}

impl<T> State<T> {
    pub fn borrow(&self) -> Ref<T> {
        self.value.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }
}
