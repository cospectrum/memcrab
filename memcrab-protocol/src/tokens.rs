use std::mem::size_of;

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

impl RequestHeader {
    pub(crate) const fn byte_size(&self) -> usize {
        Self::size_of()
    }
    pub(crate) const fn size_of() -> usize {
        1 + 2 * size_of::<u64>() + size_of::<u32>()
    }
    pub(crate) fn flag(&self) -> u8 {
        match self {
            Self::Get { .. } => 0,
            Self::Set { .. } => 1,
            Self::Delete { .. } => 2,
            Self::Clear => 3,
            Self::Ping => 4,
        }
    }
}

#[derive(Debug)]
pub enum ResponseHeader {
    Value { vlen: u64 },
    Ok,
    Error(Error), // max
    KeyNotFound,
    Pong,
}

impl ResponseHeader {
    pub(crate) const fn byte_size(&self) -> usize {
        Self::size_of()
    }
    pub(crate) const fn size_of() -> usize {
        1 + Error::size_of()
    }
    pub(crate) fn flag(&self) -> u8 {
        match self {
            Self::Value { .. } => 0,
            Self::Ok => 1,
            Self::Error(_) => 2,
            Self::KeyNotFound => 3,
            Self::Pong => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Validation { len: u64 },
    Internal { len: u64 },
}

impl Error {
    pub(crate) const fn byte_size(&self) -> usize {
        Self::size_of()
    }
    pub(crate) const fn size_of() -> usize {
        1 + size_of::<u64>()
    }
    pub(crate) fn flag(&self) -> u8 {
        match self {
            Self::Validation { .. } => 0,
            Self::Internal { .. } => 1,
        }
    }
}
