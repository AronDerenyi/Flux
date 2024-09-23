use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct State<T> {
    value: Rc<RefCell<T>>,
}

impl<T> State<T> {
    pub(super) fn new(value: Rc<RefCell<T>>) -> Self {
        State { value }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        State {
            value: self.value.clone(),
        }
    }
}

impl<T> State<T> {
    pub fn borrow(&self) -> Ref<T> {
        self.value.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }
}
