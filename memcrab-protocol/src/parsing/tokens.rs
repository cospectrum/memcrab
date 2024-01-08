use std::mem::size_of;

/// client -> server msg
#[derive(Debug)]
pub struct Request {
    pub header: RequestHeader,
    pub payload: Payload,
}

/// server -> client msg
#[derive(Debug)]
pub struct Response {
    pub header: ResponseHeader,
    pub payload: Payload,
}

#[derive(Debug)]
pub enum Payload {
    Zero,
    Key { key: String },
    Value { value: Vec<u8> },
    Pair { key: String, value: Vec<u8> },
    ErrMsg(String),
}

pub type KeyLen = u64;
pub type ValueLen = u64;
pub type Expiration = u32;

#[derive(Debug)]
pub enum RequestHeader {
    Get {
        klen: KeyLen,
    },
    Set {
        klen: KeyLen,
        vlen: ValueLen,
        expiration: Expiration,
    },
    Delete {
        klen: KeyLen,
    },
    Clear,
    Ping,
}

impl RequestHeader {
    pub const fn klen_size() -> usize {
        size_of::<KeyLen>()
    }
    pub const fn vlen_size() -> usize {
        size_of::<ValueLen>()
    }
    pub const fn expiration_size() -> usize {
        size_of::<Expiration>()
    }
    pub const fn size() -> usize {
        1 + Self::klen_size() + Self::vlen_size() + Self::expiration_size()
    }
}

#[derive(Debug)]
pub enum ResponseHeader {
    Ok,
    Error(Error),
    Value { vlen: u64 },
    KeyNotFound,
    Pong,
}

#[derive(Debug, Clone)]
pub enum Error {
    Validation { len: u64 },
    Internal { len: u64 },
}

#[cfg(test)]
mod tests {}
