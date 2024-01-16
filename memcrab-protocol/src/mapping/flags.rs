use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum RequestFlag {
    Version = 0,
    Ping = 1,
    Get = 2,
    Set = 3,
    Delete = 4,
    Clear = 5,
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
