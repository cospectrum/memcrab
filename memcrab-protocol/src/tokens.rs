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

#[derive(Debug)]
pub enum RequestHeader {
    Ping,
    Get { klen: u64 },
    Set { klen: u64, vlen: u64, exp: Seconds },
    Clear,
    Delete { klen: u64 },
}

#[derive(Debug)]
pub enum ResponseHeader {
    Error(Error),
    Pong,
    Value { vlen: u64 },
    KeyNotFound,
    Ok,
}

#[derive(Debug)]
pub enum Error {
    Validation { len: u64 },
    Internal { len: u64 },
}

#[derive(Debug, Clone, Copy)]
pub struct Seconds(pub u64);
