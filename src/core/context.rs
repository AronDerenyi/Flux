use crate::utils::{bigraph::Bigraph, id_vec::Id};
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

pub type StateKey = (Option<Id>, TypeId);
pub type States = HashMap<StateKey, Box<dyn Any>>;
pub type StateDependencies = Bigraph<Option<Id>, StateKey>;
pub type StateChanges = HashSet<StateKey>;

pub struct Context<'a> {
    id: Option<Id>,
    states: &'a mut States,
    dependencies: RefCell<&'a mut StateDependencies>,
}

impl Context<'_> {
    pub(crate) fn new<'a>(
        states: &'a mut States,
        dependencies: &'a mut StateDependencies,
    ) -> Context<'a> {
        Context {
            id: None,
            states,
            dependencies: RefCell::new(dependencies),
        }
    }

    pub(super) fn with_id<T>(&mut self, id: Id, f: impl FnOnce(&mut Context) -> T) -> T {
        self.dependencies.borrow_mut().remove_u(Some(id));
        f(&mut Context {
            id: Some(id),
            states: self.states,
            dependencies: RefCell::new(*self.dependencies.borrow_mut()),
        })
    }
}

impl Context<'_> {
    pub fn state<T: Any>(&mut self, init: impl FnOnce() -> T) -> Binding<T> {
        let binding = Binding {
            owner: self.id,
            _phantom_data: PhantomData,
        };
        self.states
            .entry(binding.into())
            .or_insert_with(|| Box::new(init()));
        binding
    }

    pub fn try_get<T: Any>(&self, binding: Binding<T>) -> Option<&T> {
        self.dependencies
            .borrow_mut()
            .add_connection(self.id, binding.into());
        self.states
            .get(&binding.into())
            .and_then(|state| state.downcast_ref())
    }

    pub fn get<T: Any>(&self, binding: Binding<T>) -> &T {
        self.try_get(binding).expect("State doesn't exist")
    }
}

pub struct ContextMut<'a> {
    id: Option<Id>,
    states: &'a mut States,
    changes: &'a mut StateChanges,
}

impl ContextMut<'_> {
    pub(crate) fn new<'a>(states: &'a mut States, changes: &'a mut StateChanges) -> ContextMut<'a> {
        ContextMut {
            id: None,
            states,
            changes,
        }
    }

    pub(super) fn with_id<T>(&mut self, id: Id, f: impl FnOnce(&mut ContextMut) -> T) -> T {
        f(&mut ContextMut {
            id: Some(id),
            states: self.states,
            changes: self.changes,
        })
    }
}

impl ContextMut<'_> {
    pub fn try_get<T: Any>(&self, binding: Binding<T>) -> Option<&T> {
        self.states
            .get(&binding.into())
            .and_then(|state| state.downcast_ref())
    }

    pub fn try_get_mut<T: Any>(&mut self, binding: Binding<T>) -> Option<&mut T> {
        self.changes.insert(binding.into());
        self.states
            .get_mut(&binding.into())
            .and_then(|state| state.downcast_mut())
    }

    pub fn get<T: Any>(&self, binding: Binding<T>) -> &T {
        self.try_get(binding).expect("State doesn't exist")
    }

    pub fn get_mut<T: Any>(&mut self, binding: Binding<T>) -> &mut T {
        self.try_get_mut(binding).expect("State doesn't exist")
    }
}

pub struct Binding<T> {
    owner: Option<Id>,
    _phantom_data: PhantomData<T>,
}

impl<T> PartialEq for Binding<T> {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
    }
}

impl<T> Eq for Binding<T> {}

impl<T> Clone for Binding<T> {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner,
            _phantom_data: self._phantom_data,
        }
    }
}

impl<T> Copy for Binding<T> {}

impl<T: Any> From<Binding<T>> for StateKey {
    fn from(binding: Binding<T>) -> StateKey {
        (binding.owner, TypeId::of::<T>())
    }
}
