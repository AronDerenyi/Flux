use crate::utils::{
    id_vec::Id,
    tracked_ref::{RefMutTracker, RefTracker, TrackedRef, TrackedRefMut},
};
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Key(Id, TypeId);

pub struct States {
    pub states: HashMap<Key, Box<dyn Any>>,
    pub changes: HashSet<Key>,
    pub dependencies: Dependencies<Id, Key>,
}

pub type StateRef<'a, T> = TrackedRef<'a, T, StateRefTracker<'a>>;
pub type StateRefMut<'a, T> = TrackedRefMut<'a, T, StateRefMutTracker<'a>>;

impl States {
    pub fn new() -> Self {
        States {
            states: HashMap::new(),
            changes: HashSet::new(),
            dependencies: Dependencies::new(),
        }
    }

    pub fn state<T: Any>(&mut self, id: Id, init: impl FnOnce() -> T) -> Binding<T> {
        let key = Key(id, TypeId::of::<T>());
        if !self.states.contains_key(&key) {
            self.states.insert(key, Box::new(init()));
        }
        Binding::new(key)
    }

    pub fn get<T: Any>(&mut self, binding: Binding<T>, id: Option<Id>) -> Option<StateRef<T>> {
        self.states
            .get_mut(&binding.key)
            .and_then(|state| state.downcast_ref())
            .map(|state| {
                TrackedRef::new(
                    state,
                    StateRefTracker {
                        key: binding.key,
                        id,
                        dependencies: &mut self.dependencies,
                    },
                )
            })
    }

    pub fn get_mut<T: Any>(
        &mut self,
        binding: Binding<T>,
        id: Option<Id>,
    ) -> Option<StateRefMut<T>> {
        self.states
            .get_mut(&binding.key)
            .and_then(|state| state.downcast_mut())
            .map(|state| {
                TrackedRefMut::new(
                    state,
                    StateRefMutTracker {
                        key: binding.key,
                        id,
                        changes: &mut self.changes,
                        dependencies: &mut self.dependencies,
                    },
                )
            })
    }

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }

    pub fn clear_dependencies(&mut self, id: Id) {
        self.dependencies.remove_dependent(id);
    }
}

pub struct StateRefTracker<'a> {
    key: Key,
    id: Option<Id>,
    dependencies: &'a mut Dependencies<Id, Key>,
}

impl RefTracker for StateRefTracker<'_> {
    fn accessed(&mut self) {
        if let Some(id) = self.id {
            self.dependencies.add(id, self.key);
        }
    }
}

pub struct StateRefMutTracker<'a> {
    key: Key,
    id: Option<Id>,
    changes: &'a mut HashSet<Key>,
    dependencies: &'a mut Dependencies<Id, Key>,
}

impl RefTracker for StateRefMutTracker<'_> {
    fn accessed(&mut self) {
        if let Some(id) = self.id {
            self.dependencies.add(id, self.key);
        }
    }
}

impl RefMutTracker for StateRefMutTracker<'_> {
    fn accessed_mut(&mut self) {
        self.changes.insert(self.key);
        self.accessed();
    }
}

pub struct Binding<T> {
    key: Key,
    _phantom_data: PhantomData<T>,
}

impl<T> Binding<T> {
    fn new(key: Key) -> Self {
        Self {
            key,
            _phantom_data: PhantomData,
        }
    }
}

impl<T> PartialEq for Binding<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for Binding<T> {}

impl<T> Clone for Binding<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            _phantom_data: self._phantom_data,
        }
    }
}

impl<T> Copy for Binding<T> {}

pub struct Dependencies<T, D> {
    dependencies: HashMap<T, HashSet<D>>,
    dependents: HashMap<D, HashSet<T>>,
}

impl<T: Eq + Hash + Clone, D: Eq + Hash + Clone> Dependencies<T, D> {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    pub fn add(&mut self, dependent: T, dependency: D) {
        self.dependencies
            .entry(dependent.clone())
            .or_insert_with(HashSet::new)
            .insert(dependency.clone());
        self.dependents
            .entry(dependency)
            .or_insert_with(HashSet::new)
            .insert(dependent);
    }

    pub fn remove_dependent(&mut self, dependent: T) {
        self.dependencies.remove(&dependent).map(|dependencies| {
            for dependency in dependencies {
                self.dependents.get_mut(&dependency).map(|dependents| {
                    dependents.remove(&dependent);
                });
            }
        });
    }

    pub fn remove_dependency(&mut self, dependency: D) {
        self.dependents.remove(&dependency).map(|dependents| {
            for dependent in dependents {
                self.dependencies.get_mut(&dependent).map(|dependencies| {
                    dependencies.remove(&dependency);
                });
            }
        });
    }

    pub fn get_dependents(&self, dependency: D) -> Option<&HashSet<T>> {
        self.dependents.get(&dependency)
    }

    pub fn get_dependencies(&self, dependent: T) -> Option<&HashSet<D>> {
        self.dependencies.get(&dependent)
    }
}
