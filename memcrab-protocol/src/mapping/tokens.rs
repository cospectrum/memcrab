use std::mem::size_of;

#[derive(Debug)]
pub enum Payload {
    Zero,
    Key(String),
    Value(Vec<u8>),
    Pair { key: String, value: Vec<u8> },
    ErrMsg(String),
}

pub type ErrMsgLen = u64;
pub type KeyLen = u64;
pub type ValueLen = u64;
pub type Expiration = u32;

#[derive(Debug, Clone, Copy)]
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
    pub fn payload_len(self) -> usize {
        match self {
            Self::Get { klen } => klen as usize,
            Self::Set {
                klen,
                vlen,
                expiration,
            } => (klen + vlen + expiration as u64) as usize,
            Self::Delete { klen } => klen as usize,
            _ => 0,
        }
    }
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
        let set_size = Self::klen_size() + Self::vlen_size() + Self::expiration_size();
        1 + set_size
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
    pub fn payload_len(self) -> usize {
        match self {
            Self::Error(e) => e.errmsg_len() as usize,
            Self::Value { vlen } => vlen as usize,
            _ => 0,
        }
    }
    pub const fn vlen_size() -> usize {
        size_of::<ValueLen>()
    }
    pub const fn size() -> usize {
        1 + ErrorHeader::size()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorHeader {
    Validation { len: ErrMsgLen },
    Internal { len: ErrMsgLen },
}

impl ErrorHeader {
    pub const fn errmsg_len(self) -> ErrMsgLen {
        match self {
            Self::Validation { len } => len,
            Self::Internal { len } => len,
        }
    }
    pub const fn size() -> usize {
        1 + size_of::<ErrMsgLen>()
    }
}

#[cfg(test)]
mod tests {}