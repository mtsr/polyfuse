use parking_lot::Mutex;
use std::{
    borrow::Borrow,
    collections::hash_map::{HashMap, RandomState},
    fmt,
    hash::{BuildHasher, Hash},
    sync::{Arc, Weak},
};

pub struct SharedMap<K, V, S = RandomState>(Mutex<HashMap<K, Arc<V>, S>>);

impl<K, V, S> fmt::Debug for SharedMap<K, V, S>
where
    K: Eq + Hash + fmt::Debug,
    V: fmt::Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SharedMap").field(&self.0).finish()
    }
}

impl<K, V, S> Default for SharedMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn default() -> Self {
        Self(Mutex::default())
    }
}

impl<K: Hash + Eq, V> SharedMap<K, V> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(Mutex::new(HashMap::with_capacity(cap)))
    }
}

impl<K, V, S> SharedMap<K, V, S> {
    #[inline(always)]
    fn with_inner<R>(&self, f: impl FnOnce(&HashMap<K, Arc<V>, S>) -> R) -> R {
        let map = self.0.lock();
        f(&*map)
    }

    #[inline(always)]
    fn with_inner_mut<R>(&self, f: impl FnOnce(&mut HashMap<K, Arc<V>, S>) -> R) -> R {
        let mut map = self.0.lock();
        f(&mut *map)
    }

    pub fn capacity(&self) -> usize {
        self.with_inner(|map| map.capacity())
    }

    pub fn len(&self) -> usize {
        self.with_inner(|map| map.len())
    }

    pub fn is_empty(&self) -> bool {
        self.with_inner(|map| map.is_empty())
    }
}

impl<K, V, S> SharedMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<Weak<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.with_inner(|map| map.get(key).map(Arc::downgrade))
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.with_inner(|map| map.contains_key(key))
    }

    pub fn insert(&self, key: K, value: V) -> Option<Arc<V>> {
        self.with_inner_mut(|map| map.insert(key, Arc::new(value)))
    }

    pub fn remove<Q: ?Sized>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.with_inner_mut(|map| map.remove(key))
    }
}
