use super::{change::Change, Child};
use crate::{
    core::{graphics::Graphics, Constraints, View},
    utils::id_vec::Id,
};
use macroquad::math::Vec2;
use std::{collections::HashMap, rc::Rc};
use std::{
    fmt::{Debug, Formatter},
    mem::swap,
};

pub struct Node {
    pub parent: Option<Id>,
    pub children: Box<[Id]>,

    pub view: Rc<dyn View>,
    pub position: Vec2,
    pub size: Vec2,
    pub graphics: Graphics,

    pub change: Change,
    // pub cache: HashMap<Constraints, (Vec2, bool) /*(size, whether it's new)*/>, // TODO: cache cleaning (one idea is that any cached size who wasn't queried in the last pass can be thrown out)
    pub new_cache: HashMap<Constraints, Vec2>,
    pub old_cache: HashMap<Constraints, Vec2>,

    pub cache_misses: usize,
}

impl Node {
    pub fn new(parent: Option<Id>, view: Rc<dyn View>) -> Self {
        Node {
            parent,
            children: Default::default(),
            view,
            position: Default::default(),
            size: Default::default(),
            graphics: Default::default(),
            change: Change::ALL,
            new_cache: Default::default(),
            old_cache: Default::default(),
            cache_misses: 0,
        }
    }

    pub fn invalidate(&mut self, change: Change) {
        self.change.add(change);
    }

    pub fn valid(&self, change: Change) -> bool {
        !self.change.contains(change)
    }

    pub fn cached_size(&mut self, constraints: Constraints) -> Option<Vec2> {
        if let Some(&size) = self.new_cache.get(&constraints) {
            return Some(size);
        } else if !self.change.contains(Change::SIZE) {
            if let Some(&size) = self.old_cache.get(&constraints) {
                self.new_cache.insert(constraints, size);
                return Some(size);
            }
        }
        self.cache_misses += 1;
        None
    }

    pub fn calculate_size(&mut self, constraints: Constraints, children: &Vec<Child>) -> Vec2 {
        let size = self.view.size(constraints, &children);
        self.new_cache.insert(constraints, size);
        size
    }

    pub fn clear(&mut self) {
        self.change.clear();
        self.old_cache.clear();
        swap(&mut self.new_cache, &mut self.old_cache);
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
        f.write_str("/")?;
        self.new_cache.len().fmt(f)?;
        f.write_str("/")?;
        self.old_cache.len().fmt(f)?;
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
We clear every cache entry that wasn't updated in this update cycle.

*/
