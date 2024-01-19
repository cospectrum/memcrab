use super::ByteSized;
use crate::MemLru;
use core::{hash::Hash, marker::PhantomData, num::NonZeroUsize};
use lru::LruCache;

#[derive(Clone)]
pub struct UnboundedLen;
#[derive(Clone)]
pub struct NoMaxByteSize;

pub type DefaultBuilder<K, V> = MemLruBuilder<UnboundedLen, NoMaxByteSize, K, V>;

#[derive(Clone)]
pub struct MemLruBuilder<MaxLen, MaxByteSize, K, V> {
    max_len: MaxLen,
    max_bytesize: MaxByteSize,
    phantom: PhantomData<(K, V)>,
}

impl<K, V> DefaultBuilder<K, V> {
    pub(crate) fn new() -> Self {
        Self {
            max_len: UnboundedLen,
            max_bytesize: NoMaxByteSize,
            phantom: PhantomData,
        }
    }
}

impl<S, K, V> MemLruBuilder<UnboundedLen, S, K, V> {
    pub fn max_len(self, max_len: usize) -> MemLruBuilder<usize, S, K, V> {
        assert!(max_len > 0);
        MemLruBuilder {
            max_len,
            max_bytesize: self.max_bytesize,
            phantom: PhantomData,
        }
    }
}

impl<L, K, V> MemLruBuilder<L, NoMaxByteSize, K, V> {
    pub fn max_bytesize(self, max_bytesize: usize) -> MemLruBuilder<L, usize, K, V> {
        MemLruBuilder {
            max_len: self.max_len,
            max_bytesize,
            phantom: PhantomData,
        }
    }
}

impl<K, V> MemLruBuilder<usize, usize, K, V>
where
    K: Hash + Eq + ByteSized,
    V: ByteSized,
{
    pub fn build(self) -> MemLru<K, V> {
        assert!(self.max_len > 0);
        let cap = NonZeroUsize::new(self.max_len).unwrap();
        let lru = LruCache::new(cap);
        MemLru::new(lru, self.max_bytesize)
    }
}

impl<K, V> MemLruBuilder<UnboundedLen, usize, K, V>
where
    K: Hash + Eq + ByteSized,
    V: ByteSized,
{
    pub fn build(self) -> MemLru<K, V> {
        let lru = LruCache::unbounded();
        MemLru::new(lru, self.max_bytesize)
    }
}
