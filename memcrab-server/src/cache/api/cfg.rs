use std::sync::Mutex;
use typed_builder::TypedBuilder;

use crate::cache::map::Map;

use super::{MemLru, Value};

#[derive(TypedBuilder)]
pub struct CacheCfg {
    segments: usize,
    #[builder(default=None, setter(strip_option))]
    max_len: Option<usize>,
    max_bytesize: usize,
}

impl CacheCfg {
    pub(super) fn map(self) -> Map<String, Value> {
        assert!(self.segments > 0);

        let new_segment = |max_bytesize: usize, max_len: Option<usize>| {
            Mutex::new(match max_len {
                Some(max_len) => {
                    if max_len == 0 {
                        MemLru::with_max_bytesize(max_bytesize)
                    } else {
                        MemLru::with_max_bytesize_and_max_len(max_bytesize, max_len)
                    }
                }
                None => MemLru::with_max_bytesize(max_bytesize),
            })
        };

        let mut segments = Vec::with_capacity(self.segments);
        let segment = new_segment(
            self.max_bytesize / self.segments + self.max_bytesize % self.segments,
            self.max_len.map(|l| l / self.segments + l % self.segments),
        );
        segments.push(segment);
        for _ in 1..self.segments {
            let segment = new_segment(
                self.max_bytesize / self.segments,
                self.max_len.map(|l| l / self.segments),
            );
            segments.push(segment);
        }
        Map::from_segments(segments)
    }
}
