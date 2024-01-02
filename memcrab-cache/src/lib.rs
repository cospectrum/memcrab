use core::num::NonZeroUsize;
use lru::LruCache;

#[derive(Debug)]
pub struct Cache {
    inner: LruCache<String, Vec<u8>>,
    maxbytes: usize,
    bytes: usize,
}

impl Cache {
    pub fn new(maxlen: NonZeroUsize, maxbytes: usize) -> Self {
        let inner = LruCache::new(maxlen);
        Self {
            inner,
            maxbytes,
            bytes: 0,
        }
    }
    pub fn maxbytes(&self) -> usize {
        self.maxbytes
    }
    pub fn bytes(&self) -> usize {
        self.bytes
    }
    pub fn maxlen(&self) -> usize {
        self.inner.cap().into()
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn size_of(key: &String, val: &Vec<u8>) -> usize {
        key.capacity() + val.capacity()
    }
    pub fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
        self.inner.get(key)
    }
    pub fn set(&mut self, key: String, val: Vec<u8>) -> Option<Vec<u8>> {
        let item_size = Self::size_of(&key, &val);
        assert!(item_size <= self.maxbytes());

        let result = self.pop(&key);

        self.kick_everyone_out(item_size);
        self.inner.put(key, val);
        self.bytes += item_size;

        assert!(self.bytes() <= self.maxbytes());
        result
    }

    fn kick_everyone_out(&mut self, item_size: usize) {
        assert!(item_size <= self.maxbytes());
        while self.cannot_fit(item_size) {
            self.pop_lru();
        }
    }
    fn pop(&mut self, key: &str) -> Option<Vec<u8>> {
        match self.inner.pop_entry(key) {
            Some((k, v)) => {
                self.bytes -= Self::size_of(&k, &v);
                Some(v)
            }
            None => None,
        }
    }
    fn pop_lru(&mut self) -> Option<(String, Vec<u8>)> {
        match self.inner.pop_lru() {
            Some((k, v)) => {
                self.bytes -= Self::size_of(&k, &v);
                Some((k, v))
            }
            None => None,
        }
    }
    fn can_fit(&self, item_size: usize) -> bool {
        debug_assert!(item_size <= self.maxbytes());
        self.bytes() + item_size <= self.maxbytes()
    }
    fn cannot_fit(&self, item_size: usize) -> bool {
        !self.can_fit(item_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let maxbytes = 1000;
        let maxlen = NonZeroUsize::new(11).unwrap();
        let cache = Cache::new(maxlen, maxbytes);
        println!("{:?}", cache);
    }
}
