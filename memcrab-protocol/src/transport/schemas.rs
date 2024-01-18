use crate::mapping::alias::{Expiration, Version};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Version(Version),
    Get(String),
    Set {
        key: String,
        value: Vec<u8>,
        expiration: Expiration,
    },
    Delete(String),
    Clear,
    Ping,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Value(Vec<u8>),
    Ok,
    Error(ErrorResponse),
    KeyNotFound,
    Pong,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ErrorResponse {
    #[error("validation error")]
    Validation(String),
    #[error("internal error")]
    Internal(String),
}
