use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum RequestFlag {
    Ping = 0,
    Get = 1,
    Set = 2,
    Delete = 3,
    Clear = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum ResponseFlag {
    Pong = 0,
    Value = 1,
    Ok = 2,
    KeyNotFound = 3,

    ValidationErr = 201,
    InternalErr = 202,
}
