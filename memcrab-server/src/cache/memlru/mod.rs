mod bytesized;

pub use bytesized::ByteSized;

use core::{borrow::Borrow, hash::Hash};
use lru::LruCache;

#[derive(Debug)]
pub struct MemLru<K, V>
where
    K: Hash + Eq,
{
    inner: LruCache<K, V>,
    max_bytesize: usize,
    bytesize: usize,
}

impl<K, V> ByteSized for MemLru<K, V>
where
    K: Hash + Eq,
{
    fn bytesize(&self) -> usize {
        self.bytesize
    }
}

impl<K, V> MemLru<K, V>
where
    K: ByteSized + Hash + Eq,
    V: ByteSized,
{
    pub fn with_max_bytesize(max_bytesize: usize) -> Self {
        let inner = LruCache::unbounded();
        Self::new(inner, max_bytesize)
    }
    pub fn with_max_bytesize_and_max_len(max_bytesize: usize, max_len: usize) -> Self {
        use core::num::NonZeroUsize;
        assert!(max_len > 0, "max_len should be > 0");
        let max_len = NonZeroUsize::new(max_len).unwrap();
        let inner = LruCache::new(max_len);
        Self::new(inner, max_bytesize)
    }
    pub(crate) fn new(inner: LruCache<K, V>, max_bytesize: usize) -> Self {
        Self {
            inner,
            max_bytesize,
            bytesize: 0,
        }
    }
    pub fn max_bytesize(&self) -> usize {
        self.max_bytesize
    }
    #[allow(unused)]
    pub fn max_len(&self) -> usize {
        self.inner.cap().into()
    }
    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn size_of(key: &K, val: &V) -> usize {
        key.bytesize() + val.bytesize()
    }

    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(key)
    }
    pub fn set(&mut self, key: K, val: V) -> Option<V> {
        let item_size = Self::size_of(&key, &val);
        assert!(item_size <= self.max_bytesize());

        let result = self.pop(&key);

        self.make_room_for(item_size);
        self.inner.put(key, val);
        self.add_bytesize(item_size);

        assert!(self.bytesize() <= self.max_bytesize());
        result
    }
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.pop(key)
    }
    pub fn clear(&mut self) {
        self.inner.clear();
        self.bytesize = 0;
    }

    fn make_room_for(&mut self, item_size: usize) {
        assert!(item_size <= self.max_bytesize());
        while self.cannot_fit(item_size) {
            self.pop_lru();
        }
    }
    fn pop<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match self.inner.pop_entry(key) {
            Some((k, v)) => {
                self.subtract_bytesize(Self::size_of(&k, &v));
                Some(v)
            }
            None => None,
        }
    }
    fn pop_lru(&mut self) -> Option<(K, V)> {
        match self.inner.pop_lru() {
            Some((k, v)) => {
                self.subtract_bytesize(Self::size_of(&k, &v));
                Some((k, v))
            }
            None => None,
        }
    }

    fn can_fit(&self, item_size: usize) -> bool {
        self.bytesize() + item_size <= self.max_bytesize()
    }
    fn cannot_fit(&self, item_size: usize) -> bool {
        !self.can_fit(item_size)
    }
    fn add_bytesize(&mut self, bytesize: usize) {
        self.bytesize += bytesize;
    }
    fn subtract_bytesize(&mut self, bytesize: usize) {
        self.bytesize -= bytesize;
    }
}
