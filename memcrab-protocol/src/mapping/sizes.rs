use super::alias::{ErrMsgLen, Expiration, KeyLen, ValueLen, Version};
use std::mem::size_of;

pub const REQUEST_VERSION_SIZE: usize = size_of::<Version>();
pub const REQUEST_KLEN_SIZE: usize = size_of::<KeyLen>();
pub const REQUEST_VLEN_SIZE: usize = size_of::<ValueLen>();
pub const REQUEST_EXP_SIZE: usize = size_of::<Expiration>();
pub const REQUEST_MAX_SIZE: usize = 1 + REQUEST_KLEN_SIZE + REQUEST_VLEN_SIZE + REQUEST_EXP_SIZE;

pub const RESPONSE_VLEN_SIZE: usize = size_of::<ValueLen>();
pub const MAX_RESPONSE_SIZE: usize = 1 + RESPONSE_VLEN_SIZE;

pub const ERRMSG_LEN_SIZE: usize = size_of::<ErrMsgLen>();

#[cfg(test)]
mod tests {}
