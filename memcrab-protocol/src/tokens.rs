// client writes this msg, server reads
#[derive(Debug)]
pub struct Request {
    pub header: RequestHeader,
    pub payload: Payload,
}

// server writes this msg, client reads
#[derive(Debug)]
pub struct Response {
    pub header: ResponseHeader,
    pub payload: Payload,
}

#[derive(Debug)]
pub enum Payload {
    Zero,
    Raw(Vec<u8>),
}

impl Payload {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::Raw(v) => v.len(),
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
    }, // max
    Delete {
        klen: u64,
    },
    Clear,
    Ping,
}

#[derive(Debug)]
pub enum ResponseHeader {
    Value { vlen: u64 },
    Ok,
    Error(Error), // max
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
