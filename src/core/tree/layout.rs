use itertools::Itertools;

use crate::utils::id_vec::Id;

use super::{change::Change, Tree};

impl Tree {
    pub fn calculate_layouts(&mut self, id: Id) {
        let node = &self.nodes[id];
        if !node.change.contains(Change::VIEW | Change::CONSTRAINTS) {
            return;
        }

        let mut child_size_changed = false;
        let child_ids = node.children.clone();
        let child_sizes = child_ids
            .iter()
            .map(|child_id| {
                self.calculate_layouts(*child_id);
                let child_node = &self[*child_id];
                child_size_changed |= child_node.change.contains(Change::SIZE);
                child_node.size
            })
            .collect_vec();

        let node = &mut self[id];
        if child_size_changed {
            node.change.add(Change::CHILD_SIZE);
        }

        let (size, child_positions) = node.view.calculate_layout(node.constraints, &child_sizes);
        if size != node.size {
            node.size = size;
            node.change.add(Change::SIZE);
        }

        for (child_id, child_position) in child_ids.iter().zip(child_positions) {
            let child_node = &mut self[*child_id];
            child_node.position = child_position;
        }
    }
}
