use super::alias::{ErrMsgLen, Expiration, KeyLen, ValueLen, Version};
use std::mem::size_of;

#[derive(Debug)]
pub enum Payload {
    Zero,
    Key(String),
    Value(Vec<u8>),
    Pair { key: String, value: Vec<u8> },
    ErrMsg(String),
}

#[derive(Debug, Clone, Copy)]
pub enum RequestHeader {
    Version(Version),
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
    pub const VERSION_SIZE: usize = size_of::<Version>();
    pub const KLEN_SIZE: usize = size_of::<KeyLen>();
    pub const VLEN_SIZE: usize = size_of::<ValueLen>();
    pub const EXP_SIZE: usize = size_of::<Expiration>();

    // Max size of the request header.
    pub const SIZE: usize = {
        let set_size = Self::KLEN_SIZE + Self::VLEN_SIZE + Self::EXP_SIZE;
        1 + set_size
    };

    pub fn payload_len(self) -> usize {
        match self {
            Self::Get { klen } => klen as usize,
            Self::Set {
                klen,
                vlen,
                expiration,
            } => (klen + vlen) as usize,
            Self::Delete { klen } => klen as usize,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseHeader {
    Ok,
    Error(ErrorHeader),
    Value { vlen: ValueLen },
    KeyNotFound,
    Pong,
}

impl ResponseHeader {
    pub const VLEN_SIZE: usize = size_of::<ValueLen>();
    pub const SIZE: usize = { 1 + ErrorHeader::SIZE };

    pub fn payload_len(self) -> usize {
        match self {
            Self::Error(e) => e.errmsg_len() as usize,
            Self::Value { vlen } => vlen as usize,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorHeader {
    Validation { len: ErrMsgLen },
    Internal { len: ErrMsgLen },
}

impl ErrorHeader {
    pub const MSG_LEN_SIZE: usize = size_of::<ErrMsgLen>();
    pub const SIZE: usize = { 1 + size_of::<ErrMsgLen>() };

    pub const fn errmsg_len(self) -> ErrMsgLen {
        match self {
            Self::Validation { len } => len,
            Self::Internal { len } => len,
        }
    }
}

#[cfg(test)]
mod tests {}
