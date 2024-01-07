#![allow(dead_code)]

/// First byte of the header.
/// Determines which PayloadInfo to parse:
/// if we see MsgKind::GetRequest, then PayloadInfo::Key should be used.
pub enum MsgKind {
    GetRequest = 0,
    SetRequest,
    DeleteRequest,
    ClearRequest,

    ClientErrorResponse,
    ServerErrorResponse,
    ValueResponse,
    KeyMissResponse, // get response
    SetSuccessResponse,
    DeleteSuccessResponse, // delete response
    ClearSuccessResponse,  // clear response

    Ping,
    Pong,
}

/// The next 0..5 bytes as a part of the header.
pub enum PayloadInfo {
    None,                                // clear
    Key { klen: u8 },                    // get/delete/add
    KeyAndValue { klen: u8, vlen: u64 }, // set/append
}

pub struct Header {
    kind: MsgKind,
    payload_info: PayloadInfo,
}

/// The payload of variable length.
pub enum Payload {
    None,                                         // clear
    Key { key: Vec<u8> },                         // get/delete/add
    KeyAndValue { key: Vec<u8>, value: Vec<u8> }, // set/append
}

/// A complete frame returned from read_msg (method of MemcrabChannel), costs 2 or 1 read_exact
pub struct Msg {
    header: Header,
    payload: Payload,
}

/// A high-level command passed to the server.
pub enum Cmd {
    Get { key: String },
    Set { key: String, value: Vec<u8> },
    Delete { key: String },
    Clear,
}
