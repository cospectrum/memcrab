use super::alias::{Expiration, KeyLen, ValueLen, Version};
use std::mem::size_of;

// Header fields sizes.
pub const VERSION_SIZE: usize = size_of::<Version>();
pub const KLEN_SIZE: usize = size_of::<KeyLen>();
pub const VLEN_SIZE: usize = size_of::<ValueLen>();
pub const EXP_SIZE: usize = size_of::<Expiration>();

// We manually calculate the longest possible header and make it fixed.
pub const MAX_HEADER_SIZE: usize = 1 + KLEN_SIZE + VLEN_SIZE + EXP_SIZE;
