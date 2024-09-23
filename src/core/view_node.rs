use super::{Constraints, Layout};
use crate::View;

pub struct ViewNode {
    pub view: Box<dyn View>,
    pub children: Box<[ViewNode]>,
    pub constraints: Constraints,
    pub layout: Layout,
}

impl ViewNode {
    pub fn new(view: impl View) -> Self {
        ViewNode {
            view: Box::new(view),
            children: Box::new([]),
            constraints: Constraints::default(),
            layout: Layout::default(),
        }
    }
}
