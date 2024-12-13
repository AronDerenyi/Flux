#![allow(unused)]

use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
};

pub trait RefTracker {
    fn accessed(&mut self);
}

pub trait RefMutTracker: RefTracker {
    fn accessed_mut(&mut self);
}

pub struct TrackedRef<'a, V, T: RefTracker> {
    value: &'a V,
    tracker: T,
    accessed: Cell<bool>,
}

impl<'a, V, T: RefTracker> TrackedRef<'a, V, T> {
    pub fn new(value: &'a V, tracker: T) -> Self {
        Self {
            value,
            tracker,
            accessed: Cell::new(false),
        }
    }
}

impl<V, T: RefTracker> Deref for TrackedRef<'_, V, T> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.accessed.set(true);
        &self.value
    }
}

impl<V, T: RefTracker> Drop for TrackedRef<'_, V, T> {
    fn drop(&mut self) {
        if self.accessed.get() {
            self.tracker.accessed();
        }
    }
}

pub struct TrackedRefMut<'a, V, T: RefMutTracker> {
    value: &'a mut V,
    tracker: T,
    accessed: Cell<bool>,
    accessed_mut: bool,
}

impl<'a, V, T: RefMutTracker> TrackedRefMut<'a, V, T> {
    pub fn new(value: &'a mut V, tracker: T) -> Self {
        Self {
            value,
            tracker,
            accessed: Cell::new(false),
            accessed_mut: false,
        }
    }
}

impl<V, T: RefMutTracker> Deref for TrackedRefMut<'_, V, T> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.accessed.set(true);
        &self.value
    }
}

impl<V, T: RefMutTracker> DerefMut for TrackedRefMut<'_, V, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.accessed_mut = true;
        &mut self.value
    }
}

impl<V, T: RefMutTracker> Drop for TrackedRefMut<'_, V, T> {
    fn drop(&mut self) {
        if self.accessed.get() {
            self.tracker.accessed();
        }
        if self.accessed_mut {
            self.tracker.accessed_mut();
        }
    }
}

mod tests {
    use super::*;

    impl RefTracker for &mut bool {
        fn accessed(&mut self) {
            **self = true;
        }
    }

    impl RefTracker for (&mut bool, &mut bool) {
        fn accessed(&mut self) {
            *self.0 = true;
        }
    }

    impl RefMutTracker for (&mut bool, &mut bool) {
        fn accessed_mut(&mut self) {
            *self.1 = true;
        }
    }

    #[test]
    fn nothing() {
        let value = 0;
        let mut accessed = false;
        {
            let _ = TrackedRef::new(&value, &mut accessed);
        }
        assert_eq!(value, 0);
        assert_eq!(accessed, false);
    }

    #[test]
    fn get() {
        let value = 0;
        let mut accessed = false;
        {
            let value = TrackedRef::new(&value, &mut accessed);
            let _ = *value;
        }
        assert_eq!(value, 0);
        assert_eq!(accessed, true);
    }

    #[test]
    fn nothing_mut() {
        let mut value = 0;
        let mut accessed = false;
        let mut accessed_mut = false;
        {
            let _ = TrackedRefMut::new(&mut value, (&mut accessed, &mut accessed_mut));
        }
        assert_eq!(value, 0);
        assert_eq!(accessed, false);
        assert_eq!(accessed_mut, false);
    }

    #[test]
    fn get_mut() {
        let mut value = 0;
        let mut accessed = false;
        let mut accessed_mut = false;
        {
            let value = TrackedRefMut::new(&mut value, (&mut accessed, &mut accessed_mut));
            let _ = *value;
        }
        assert_eq!(value, 0);
        assert_eq!(accessed, true);
        assert_eq!(accessed_mut, false);
    }

    #[test]
    fn set_mut() {
        let mut value = 0;
        let mut accessed = false;
        let mut accessed_mut = false;
        {
            let mut value = TrackedRefMut::new(&mut value, (&mut accessed, &mut accessed_mut));
            *value = 1;
        }
        assert_eq!(value, 1);
        assert_eq!(accessed, false);
        assert_eq!(accessed_mut, true);
    }

    #[test]
    fn get_set_mut() {
        let mut value = 0;
        let mut accessed = false;
        let mut accessed_mut = false;
        {
            let mut value = TrackedRefMut::new(&mut value, (&mut accessed, &mut accessed_mut));
            let _ = *value;
            *value = 1;
        }
        assert_eq!(value, 1);
        assert_eq!(accessed, true);
        assert_eq!(accessed_mut, true);
    }

    #[test]
    fn increment_mut() {
        let mut value = 0;
        let mut accessed = false;
        let mut accessed_mut = false;
        {
            let mut value = TrackedRefMut::new(&mut value, (&mut accessed, &mut accessed_mut));
            *value += 1;
        }
        assert_eq!(value, 1);
        assert_eq!(accessed, false);
        assert_eq!(accessed_mut, true);
    }

    #[test]
    fn update_mut() {
        let mut value = 1;
        let mut accessed = false;
        let mut accessed_mut = false;
        {
            let mut value = TrackedRefMut::new(&mut value, (&mut accessed, &mut accessed_mut));
            *value = *value * 2 + 1;
        }
        assert_eq!(value, 3);
        assert_eq!(accessed, true);
        assert_eq!(accessed_mut, true);
    }
}
