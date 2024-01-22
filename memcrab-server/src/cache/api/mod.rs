mod cfg;
mod value;

use std::num::NonZeroU32;

use super::{ByteSized, Map, MemLru};
use value::Value;

pub use cfg::CacheCfg;

pub struct Cache {
    inner: Map<String, Value>,
}

impl Cache {
    pub fn new(inner: Map<String, Value>) -> Self {
        Cache { inner }
    }
}

impl From<CacheCfg> for Cache {
    fn from(cfg: CacheCfg) -> Self {
        Self::new(cfg.map())
    }
}

impl Cache {
    pub fn set(&self, key: String, value: Vec<u8>) {
        self._set(key, Value::new(value))
    }
    pub fn set_with_expiration(&self, key: String, value: Vec<u8>, exp: NonZeroU32) {
        self._set(key, Value::with_expiration(value, exp))
    }
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self._get(key)
    }
    pub fn remove(&self, key: &str) -> Option<Vec<u8>> {
        match self.inner.remove(key) {
            Some(val) => {
                if val.expired() {
                    None
                } else {
                    Some(val.into_vec())
                }
            }
            None => None,
        }
    }
    pub fn clear(&self) {
        self.inner.clear()
    }
}

impl Cache {
    fn _set(&self, key: String, value: Value) {
        self.inner.set(key, value);
    }
    fn _get(&self, key: &str) -> Option<Vec<u8>> {
        // Don't forget that the mutex is locked until the function returns.
        // Accessing the same key inside will result in a deadlock.
        let f = |opt: Option<&Value>| {
            if let Some(val) = opt {
                if val.expired() {
                    LazyVal::Expired
                } else {
                    LazyVal::Val(val.clone().into_vec())
                }
            } else {
                LazyVal::NotFound
            }
        };
        match self.inner.get_and_then(key, f) {
            LazyVal::Val(val) => Some(val),
            LazyVal::Expired => {
                self.inner.remove(key);
                None
            }
            LazyVal::NotFound => None,
        }
    }
}

enum LazyVal {
    Expired,
    NotFound,
    Val(Vec<u8>),
}
