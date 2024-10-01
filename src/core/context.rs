use crate::utils::id_vec::Id;
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use super::State;

pub struct Context<'a> {
    id: Id,
    states: &'a mut HashMap<Id, Rc<dyn Any>>,
}

impl Context<'_> {
    pub(super) fn new<'a>(id: Id, states: &'a mut HashMap<Id, Rc<dyn Any>>) -> Context<'a> {
        Context { id, states }
    }
}

impl Context<'_> {
    pub fn state<T: Any>(&mut self, init: impl FnOnce() -> T) -> State<T> {
        State::new(
            if let Some(state) = self
                .states
                .get(&self.id)
                .and_then(|s| s.clone().downcast::<RefCell<T>>().ok())
            {
                state
            } else {
                let state = Rc::new(RefCell::new(init()));
                self.states.insert(self.id, state.clone());
                state
            },
        )
    }
}
