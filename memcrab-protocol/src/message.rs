use crate::alias::{Expiration, Version};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Request(Request),
    Response(Response),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    Value(Vec<u8>),
    Ok,
    Error(String),
    KeyNotFound,
    Pong,
}
