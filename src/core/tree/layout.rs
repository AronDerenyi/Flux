use super::{change::Change, Tree};
use crate::{core::Layout, utils::id_vec::Id};
use itertools::Itertools;
use macroquad::math::Vec2;

impl Tree {
    pub fn calculate_layouts(&mut self, id: Id) {
        let node = &self[id];
        if !node
            .change
            .contains(Change::VIEW | Change::CHILD_CONSTRAINTS | Change::LAYOUT)
        {
            return;
        }

        let child_ids = node.children.clone();
        let child_constraints = child_ids
            .iter()
            .map(|child_id| self[*child_id].constraints)
            .collect_vec();

        let child_layouts = node.view.calculate_layouts(node.layout, &child_constraints);

        for (child_id, child_layout) in child_ids.iter().zip(child_layouts) {
            let child_node = &mut self[*child_id];
            if child_layout != child_node.layout {
                child_node.layout = child_layout;
                child_node.change.add(Change::LAYOUT);
            }
            self.calculate_layouts(*child_id);
        }
    }

    pub fn calculate_root_layout(&mut self) {
        let root_id = self.root;
        let node = &mut self[root_id];
        let layout = Layout {
            position: Vec2::ZERO,
            size: node.constraints.size,
        };
        if layout != node.layout {
            node.layout = layout;
            node.change.add(Change::LAYOUT);
        }
    }
}
