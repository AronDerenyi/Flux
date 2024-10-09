use super::{change::Change, Tree};
use crate::utils::id_vec::Id;
use itertools::Itertools;

impl Tree {
    pub fn calculate_size_hints(&mut self, id: Id) {
        let node = &self[id];
        if !node.change.contains(Change::VIEW) {
            return;
        }

        let mut child_size_hints_changed = false;
        let child_size_hints = node
            .children
            .to_vec()
            .into_iter()
            .map(|child_id| {
                self.calculate_size_hints(child_id);
                let child_node = &self[child_id];
                child_size_hints_changed |= child_node.change.contains(Change::SIZE_HINT);
                child_node.size_hint
            })
            .collect_vec();

        let node = &mut self[id];
        if child_size_hints_changed {
            node.change.add(Change::CHILD_SIZE_HINT);
        }

        let size_hint = node.view.calculate_size_hint(&child_size_hints);
        if size_hint != node.size_hint {
            node.size_hint = size_hint;
            node.change.add(Change::SIZE_HINT);
        }
    }
}
