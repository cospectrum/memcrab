use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum RequestKind {
    Ping = 0,
    Get = 1,
    Set = 2,
    Delete = 3,
    Clear = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum ResponseStatus {
    Success = 0,
    Error = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum SuccessfulResponse {
    Pong = 0,
    Value = 1,
    Executed = 2,
    KeyNotFound = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum ErrorResponse {
    Validation = 0,
    Internal = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ok = 0;
        match ok.try_into().unwrap() {
            ResponseStatus::Success => (),
            variant => panic!("got unexpected variant: {:?}", variant),
        }
    }
}
