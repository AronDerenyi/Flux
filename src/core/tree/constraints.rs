use super::{change::Change, Tree};
use crate::utils::id_vec::Id;
use itertools::Itertools;

impl Tree {
    pub fn calculate_constraints(&mut self, id: Id) {
        let node = &self[id];
        if !node.change.contains(Change::VIEW) {
            return;
        }

        let mut child_constraints_changed = false;
        let child_constraints = node
            .children
            .to_vec()
            .into_iter()
            .map(|child_id| {
                self.calculate_constraints(child_id);
                let child_node = &self[child_id];
                child_constraints_changed |= child_node.change.contains(Change::CONSTRAINTS);
                child_node.constraints
            })
            .collect_vec();

        let node = &mut self[id];
        if child_constraints_changed {
            node.change.add(Change::CHILD_CONSTRAINTS);
        }

        let constraints = node.view.calculate_constraints(&child_constraints);
        if constraints != node.constraints {
            node.constraints = constraints;
            node.change.add(Change::CONSTRAINTS);
        }
    }
}
