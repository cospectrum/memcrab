#![allow(dead_code)]

pub enum Header {
    GetRequest { klen: u8 },
    SetRequest { klen: u8, vlen: u64, exp: u64 },
    DeleteRequest { klen: u8, vlen: u64 },
    ClearRequest,

    ClientErrorResponse,
    ServerErrorResponse,
    ValueResponse { vlen: u64 },
    KeyMissResponse,
    SetSuccessResponse,
    DeleteSuccessResponse,
    ClearSuccessResponse,

    Ping,
    Pong,
}

pub struct Msg {
    header: Header,
    payload: Option<Vec<u8>>,
}

/// A high-level command passed to the server.
pub enum Cmd {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: Vec<u8>,
        exp: u64,
    },
    Delete {
        key: String,
    },
    Clear,
}
