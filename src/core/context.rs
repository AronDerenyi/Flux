use super::{
    states::{self, Binding},
    StateRef, StateRefMut, States,
};
use crate::utils::id_vec::Id;
use std::{
    any::Any,
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct Context<'a> {
    id: Option<Id>,
    states: &'a mut States,
}

impl Context<'_> {
    pub(crate) fn new<'a>(states: &'a mut States) -> Context<'a> {
        Context { id: None, states }
    }

    pub(super) fn with_id<T>(&mut self, id: Id, f: impl FnOnce(&mut Context) -> T) -> T {
        f(&mut Context { id: Some(id), states: self.states })
    }
}

impl Context<'_> {
    pub fn state<T: Any>(&mut self, init: impl FnOnce() -> T) -> Binding<T> {
        self.states.state(self.id.unwrap(), init)
    }

    pub fn get<T: Any>(&mut self, binding: Binding<T>) -> StateRef<T> {
        self.states.get(binding, self.id).unwrap()
    }
}

pub struct ContextMut<'a> {
    id: Option<Id>,
    states: &'a mut States,
}

impl ContextMut<'_> {
    pub(crate) fn new<'a>(states: &'a mut States) -> ContextMut<'a> {
        ContextMut { id: None, states }
    }

    pub(super) fn with_id<T>(&mut self, id: Id, f: impl FnOnce(&mut ContextMut) -> T) -> T {
        f(&mut ContextMut { id: Some(id), states: self.states })
    }
}

impl ContextMut<'_> {
    pub fn get<T: Any>(&mut self, binding: Binding<T>) -> StateRefMut<T> {
        self.states.get_mut(binding, None).unwrap()
    }
}
