use crate::{
    core::{graphics::Graphics, Painter},
    utils::id_vec::Id,
};

use super::{change::Change, Tree};

impl Tree {
    pub fn calculate_graphics(&mut self, id: Id) {
        let node = &mut self.nodes[id];
        if !node.change.contains(Change::ALL) {
            return;
        }

        if node.change.contains(Change::VIEW | Change::SIZE) {
            let mut painter = Painter::new();
            node.view.draw(node.size, &mut painter);
            node.graphics = Graphics::from_painter(painter);
        }

        for child_id in node.children.clone() {
            self.calculate_graphics(child_id);
        }
    }
}
