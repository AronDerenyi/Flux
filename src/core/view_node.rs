use super::{Constraints, Layout};
use crate::View;

pub struct ViewNode {
    pub id: u64,
    pub view: Box<dyn View>,
    pub children: Box<[ViewNode]>,
    pub constraints: Constraints,
    pub layout: Layout,
}

impl ViewNode {
    pub fn new(id: u64, view: Box<dyn View>) -> Self {
        ViewNode {
            id,
            view,
            children: Box::new([]),
            constraints: Constraints::default(),
            layout: Layout::default(),
        }
    }
}
