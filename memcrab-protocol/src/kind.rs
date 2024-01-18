use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum MessageKind {
    VersionRequest = 0,
    PingRequest = 1,
    GetRequest = 2,
    SetRequest = 3,
    DeleteRequest = 4,
    ClearRequest = 5,

    PongResponse = 128,
    OkResponse = 129,
    ValueResponse = 130,
    KeyNotFoundResponse = 131,
    ErrorResponse = 132,
}
