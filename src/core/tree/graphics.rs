use crate::utils::id_vec::Id;

use super::{change::Change, Tree};

impl Tree {
    pub fn calculate_graphics(&mut self, id: Id) {
        let node = &mut self.nodes[id];
        if !node.change.contains(Change::ALL) {
            return;
        }

        if node.change.contains(Change::VIEW | Change::LAYOUT) {
            node.graphics = node.view.draw(node.layout);
        }

        for child_id in node.children.clone() {
            self.calculate_graphics(child_id);
        }
    }
}
