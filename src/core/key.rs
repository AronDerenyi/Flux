use std::{
    any::Any,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use dyn_clone::DynClone;

// pub enum Key<K: Hash> {
//     Global(K),
//     Local(K),
//     Path,
// }

// #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
// pub struct ID(u64);

// impl ID {
//     pub fn from_key<K: Hash>(key: &Key<K>) -> Self {
//         let mut hasher = DefaultHasher::new();
//         match key {
//             Key::Global(key) => key.hash(&mut hasher),
//             Key::Local(key) => {
//                 0.hash(&mut hasher);
//                 key.hash(&mut hasher);
//             }
//             Key::Path => {
//                 0.hash(&mut hasher);
//                 0.hash(&mut hasher);
//             }
//         }
//         ID(hasher.finish())
//     }

//     pub fn from_child_key<K: Hash>(key: &Key<K>, parent: ID, index: usize) -> Self {
//         let mut hasher = DefaultHasher::new();
//         match key {
//             Key::Global(key) => key.hash(&mut hasher),
//             Key::Local(key) => {
//                 parent.hash(&mut hasher);
//                 key.hash(&mut hasher);
//             }
//             Key::Path => {
//                 parent.hash(&mut hasher);
//                 index.hash(&mut hasher);
//             }
//         }
//         ID(hasher.finish())
//     }
// }

trait Key {}

impl PartialEq for dyn Key {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
