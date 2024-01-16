use crate::mapping::alias::{Expiration, Version};
use thiserror::Error;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Response {
    Value(Vec<u8>),
    Ok,
    Error(ErrorResponse),
    KeyNotFound,
    Pong,
}

#[derive(Error, Debug, Clone)]
pub enum ErrorResponse {
    #[error("validation error")]
    Validation(String),
    #[error("internal error")]
    Internal(String),
}
