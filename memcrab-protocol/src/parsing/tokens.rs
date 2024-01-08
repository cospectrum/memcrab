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
    Data(Vec<u8>),
}

impl Payload {
    #[allow(unused)]
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::Data(v) => v.len(),
        }
    }
}

#[derive(Debug)]
pub enum RequestHeader {
    Get {
        klen: u64,
    },
    Set {
        klen: u64,
        vlen: u64,
        expiration: u32,
    },
    Delete {
        klen: u64,
    },
    Clear,
    Ping,
}

impl RequestHeader {
    pub fn klen_size() -> usize {
        8
    }
    pub fn vlen_size() -> usize {
        8
    }
    pub fn expiration_size() -> usize {
        4
    }
    pub fn size() -> usize {
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
