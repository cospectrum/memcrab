use serde::{Deserialize, Serialize};

// client writes this msg, server reads
#[derive(Debug)]
pub struct Request {
    header: RequestHeader,
    payload: Option<Vec<u8>>,
}

// server writes this msg, client reads
#[derive(Debug)]
pub struct Response {
    header: ResponseHeader,
    payload: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestHeader {
    Ping,
    Get { klen: u64 },
    Set { klen: u64, vlen: u64, expiration: Seconds },
    Clear,
    Delete { klen: u64 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseHeader {
    Error(Error),
    Pong,
    Value { vlen: u64 },
    KeyNotFound,
    Ok,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    Validation { len: u64 },
    Internal { len: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Seconds(pub u64);
