use std::hash::{DefaultHasher, Hash, Hasher};

pub enum Key<K: Hash> {
    Global(K),
    Local(K),
    Path,
}

impl<K: Hash> Key<K> {
    pub fn id(&self, parent: u64, index: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        match self {
            Key::Global(key) => key.hash(&mut hasher),
            Key::Local(key) => {
                parent.hash(&mut hasher);
                key.hash(&mut hasher);
            }
            Key::Path => {
                parent.hash(&mut hasher);
                index.hash(&mut hasher);
            }
        }
        hasher.finish()
    }
}
