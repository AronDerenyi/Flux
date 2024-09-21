use dyn_clone::DynClone;
use macroquad::math::Vec2;

use crate::view_tree::Context;
use std::{any::type_name, rc::Rc};

#[allow(unused)]
pub trait View: 'static + DynClone {
    fn get_children(&self, ctx: &mut Context) -> Box<[Box<dyn View>]> {
        Default::default()
    }

    fn get_constraints(&self, child_constraints: &[Constraints]) -> Constraints;

    fn get_children_layouts(
        &self,
        layout: Layout,
        child_constraints: &[Constraints],
    ) -> Box<[Layout]> {
        Default::default()
    }

    fn interact(&self) -> bool {
        false
    }

    fn get_debug_string(&self) -> &str {
        let mut type_name = type_name::<Self>();
        if let Some(generic_start) = type_name.find("<") {
            type_name = &type_name[..generic_start];
        }
        if let Some(path_end) = type_name.rfind("::") {
            type_name = &type_name[path_end + 2..];
        }
        type_name
    }
}

dyn_clone::clone_trait_object!(View);

#[derive(Debug, Default, Clone, Copy)]
pub struct Constraints {
    pub size: Vec2,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Layout {
    pub position: Vec2,
    pub size: Vec2,
}

impl Layout {
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.y >= self.position.y
            && point.x < self.position.x + self.size.x
            && point.y < self.position.y + self.size.y
    }
}

#[derive(Clone)]
pub struct ViewBuilder<V: View> {
    builder: Rc<dyn Fn() -> V>,
}

impl<V: View + Clone> ViewBuilder<V> {
    pub fn new<F: Fn() -> V + 'static>(builder: F) -> Self {
        Self {
            builder: Rc::new(builder),
        }
    }

    pub fn from_view(view: V) -> Self {
        Self::new(move || view.clone())
    }

    pub fn build(&self) -> V {
        (self.builder)()
    }
}

#[derive(Clone)]
pub struct ContentBuilder {
    builder: Rc<dyn Fn() -> Box<[Box<dyn View>]>>,
}

impl ContentBuilder {
    pub fn new<F: Fn() -> Box<[Box<dyn View>]> + 'static>(builder: F) -> Self {
        Self {
            builder: Rc::new(builder),
        }
    }

    pub fn from_slice<const N: usize>(views: [Box<dyn View>; N]) -> Self {
        Self::new(move || Box::new(views.clone()))
    }

    pub fn from_Boxed_slice(views: Box<[Box<dyn View>]>) -> Self {
        Self::new(move || views.clone())
    }

    pub fn build(&self) -> Box<[Box<dyn View>]> {
        (self.builder)()
    }
}
