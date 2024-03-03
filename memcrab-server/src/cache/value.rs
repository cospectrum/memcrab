use std::{num::NonZeroU32, time::Instant};

use super::ByteSized;

#[derive(Clone, Debug)]
struct Clock {
    start: Instant,
    expiration: u32,
}

impl Clock {
    fn start_timing(expiration: NonZeroU32) -> Self {
        let start = Instant::now();
        Self {
            start,
            expiration: expiration.into(),
        }
    }
    fn expired(&self) -> bool {
        let passed = self.start.elapsed().as_secs() as u32;
        passed > self.expiration
    }
}

#[derive(Clone, Debug)]
pub struct Value {
    inner: Vec<u8>,
    clock: Option<Clock>,
}

impl Value {
    pub fn new(inner: Vec<u8>) -> Self {
        Self { inner, clock: None }
    }
    pub fn with_expiration(inner: Vec<u8>, expiration: NonZeroU32) -> Self {
        let clock = Some(Clock::start_timing(expiration));
        Self { inner, clock }
    }
    pub fn expired(&self) -> bool {
        self.clock.as_ref().map(|c| c.expired()).unwrap_or(false)
    }
    pub fn into_vec(self) -> Vec<u8> {
        self.inner
    }
}

impl ByteSized for Value {
    fn bytesize(&self) -> usize {
        self.inner.bytesize()
    }
}
