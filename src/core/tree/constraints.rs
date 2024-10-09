use super::{change::Change, Tree};
use crate::{core::Constraints, utils::id_vec::Id};
use itertools::Itertools;
use macroquad::{
    math::Vec2,
    miniquad::window::screen_size,
    window::{screen_height, screen_width},
};

impl Tree {
    pub fn calculate_constraints(&mut self, id: Id) {
        let node = &self[id];
        if !node
            .change
            .contains(Change::VIEW | Change::CONSTRAINTS | Change::CHILD_SIZE_HINT)
        {
            return;
        }

        let child_ids = node.children.clone();
        let child_size_hints = child_ids
            .iter()
            .map(|child_id| self[*child_id].size_hint)
            .collect_vec();

        let child_constraints = node
            .view
            .calculate_constraints(node.constraints, &child_size_hints);

        for (child_id, child_constraints) in child_ids.iter().zip(child_constraints) {
            let child_node = &mut self[*child_id];
            if child_constraints != child_node.constraints {
                child_node.constraints = child_constraints;
                child_node.change.add(Change::CONSTRAINTS);
            }
            self.calculate_constraints(*child_id);
        }
    }

    pub fn calculate_root_constraints(&mut self) {
        let root_id = self.root;
        let node = &mut self[root_id];
        let constraints = Constraints {
            min_width: node.size_hint.min_width,
            min_height: node.size_hint.min_height,
            max_width: f32::min(node.size_hint.max_width, screen_width()),
            max_height: f32::min(node.size_hint.max_height, screen_height()),
        };
        if constraints != node.constraints {
            node.constraints = constraints;
            node.change.add(Change::CONSTRAINTS);
        }
    }
}
