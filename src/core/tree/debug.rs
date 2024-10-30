use super::Tree;
use crate::utils::id_vec::Id;

impl Tree {
    pub fn debug_print(&self, id: Id, indent: String) {
        let node = self.nodes[id].borrow();

        println!("{:?}: {:?}", node, id);
        for (index, child_index) in node.children.iter().enumerate() {
            let last = index == node.children.len() - 1;
            print!("{}{} ", indent, if last { "╚" } else { "╠" });
            self.debug_print(
                *child_index,
                if last {
                    indent.clone() + "  ".into()
                } else {
                    indent.clone() + "║ ".into()
                },
            );
        }
    }
}
