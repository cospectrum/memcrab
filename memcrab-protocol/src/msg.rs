use crate::alias::Expiration;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Request(Request),
    Response(Response),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Request {
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
