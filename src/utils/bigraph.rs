#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Bigraph<U, V> {
    empty_us: HashSet<U>,
    empty_vs: HashSet<V>,
    uv_edges: HashMap<U, HashSet<V>>,
    vu_edges: HashMap<V, HashSet<U>>,
}

impl<U: Eq + Hash + Clone, V: Eq + Hash + Clone> Bigraph<U, V> {
    pub fn new() -> Self {
        Self {
            empty_us: HashSet::new(),
            empty_vs: HashSet::new(),
            uv_edges: HashMap::new(),
            vu_edges: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, u: U, v: V) {
        self.uv_edges
            .entry(u.clone())
            .or_insert_with(HashSet::new)
            .insert(v.clone());
        self.vu_edges
            .entry(v)
            .or_insert_with(HashSet::new)
            .insert(u);
    }

    pub fn remove_u(&mut self, u: U) {
        if let Some(connected_vs) = self.uv_edges.remove(&u) {
            for v in connected_vs {
                if let Some(connected_us) = self.vu_edges.get_mut(&v) {
                    connected_us.remove(&u);
                    if connected_us.is_empty() {
                        self.vu_edges.remove(&v);
                    }
                }
            }
        }
    }

    pub fn remove_v(&mut self, v: V) {
        if let Some(connected_us) = self.vu_edges.remove(&v) {
            for u in connected_us {
                if let Some(connected_vs) = self.uv_edges.get_mut(&u) {
                    connected_vs.remove(&v);
                    if connected_vs.is_empty() {
                        self.uv_edges.remove(&u);
                    }
                }
            }
        }
    }

    pub fn get_u_connections(&self, u: U) -> &HashSet<V> {
        self.uv_edges.get(&u).unwrap_or(&self.empty_vs)
    }

    pub fn get_v_connections(&self, v: V) -> &HashSet<U> {
        self.vu_edges.get(&v).unwrap_or(&self.empty_us)
    }
}
