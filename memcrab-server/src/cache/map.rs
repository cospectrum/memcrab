use super::{ByteSized, MemLru};
use core::{borrow::Borrow, hash::Hash};
use std::{
    collections::hash_map::RandomState,
    sync::{Mutex, MutexGuard},
};

type Segment<K, V> = Mutex<MemLru<K, V>>;
type Segments<K, V> = Vec<Segment<K, V>>;

pub struct Map<K, V, H = RandomState>
where
    K: Hash + Eq,
{
    segments: Segments<K, V>,
    hasher: H,
}

impl<K, V> Map<K, V>
where
    K: Hash + Eq + ByteSized,
    V: ByteSized + Clone,
{
    pub fn from_segments(segments: Segments<K, V>) -> Self {
        Self {
            segments,
            hasher: RandomState::default(),
        }
    }
    pub fn set(&self, key: K, val: V) -> Option<V> {
        let mut segment = self.lock_segment_for_key(&key);
        segment.set(key, val)
    }
    #[allow(unused)]
    pub fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let f = |opt: Option<&V>| opt.cloned();
        self.get_and_then(key, f)
    }
    pub fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut segment = self.lock_segment_for_key(&key);
        segment.remove(key)
    }
    pub fn clear(&self) {
        self.segments.iter().for_each(|seg| {
            let mut seg = seg.lock().unwrap();
            seg.clear();
        })
    }
    pub fn get_and_then<Q, F, T>(&self, key: &Q, f: F) -> T
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        F: FnOnce(Option<&V>) -> T,
    {
        let mut segment = self.lock_segment_for_key(&key);
        let opt = segment.get(key);
        f(opt)
    }

    fn lock_segment_for_key<T: Hash>(&self, key: &T) -> MutexGuard<MemLru<K, V>> {
        let at = self.determine_segment(key);
        self.segments[at].lock().unwrap()
    }
    fn determine_segment<T: Hash>(&self, key: &T) -> usize {
        let hash = self.hash(key);
        let idx = hash % self.segments.len() as u64;
        idx as usize
    }
    fn hash<T: Hash>(&self, item: &T) -> u64 {
        use core::hash::{BuildHasher, Hasher};

        let mut hasher = self.hasher.build_hasher();
        item.hash(&mut hasher);
        hasher.finish()
    }
}
