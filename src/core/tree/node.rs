use super::change::Change;
use crate::{
    core::{graphics::Graphics, Constraints, View},
    utils::id_vec::Id,
};
use macroquad::math::Vec2;
use std::fmt::{Debug, Formatter};
use std::{collections::HashMap, rc::Rc};

pub struct Node {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,
    pub change: Change,

    pub view: Rc<dyn View>,
    pub cache: HashMap<Constraints, (Vec2, bool) /*(size, whether it's new)*/>, // TODO: cache cleaning (one idea is that any cached size who wasn't queried in the last pass can be thrown out)
    pub position: Vec2,
    pub size: Vec2,
    pub graphics: Graphics,

    pub cache_misses: usize,
}

impl Node {
    pub fn new(parent: Option<Id>, view: Rc<dyn View>) -> Self {
        Node {
            parent,
            children: Default::default(),
            change: Change::ALL,
            view,
            cache: Default::default(),
            position: Default::default(),
            size: Default::default(),
            graphics: Default::default(),
            cache_misses: 0,
        }
    }

    pub fn clear(&mut self) {
        self.change.clear();
        for (_, new) in self.cache.values_mut() {
            *new = false;
        }
        self.cache_misses = 0;
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.view.debug_name());
        f.write_str("(")?;
        size_of_val(&*self.view).fmt(f)?;
        f.write_str(", ")?;
        self.size.fmt(f)?;
        f.write_str(", ")?;
        self.change.fmt(f)?;
        f.write_str(", ")?;
        self.cache_misses.fmt(f)?;
        f.write_str(", ")?;
        self.cache.len().fmt(f)?;
        f.write_str(")")
    }
}

/*

After state changed:

Rebuild children (and children's children) if their view changed.
If the view changed, invalidate the size cache.

Validate children's size cache by checking if the cached sizes are equal to the generated sizes.
If any child's size cache still invalid, invalidate the parent size cache and repeat from the previous step.




A cache miss can occur when:
- The cache doesn't contain the queried constraint.
- The size is invalidated and the cached size wasn't updated in this update cycle.

Only if a cache miss occurs, the children's cache is queried.
In the clear step, we traverse down until we find a node who wasn't invalidated and it's parent hasn't had a cache miss.

*/
