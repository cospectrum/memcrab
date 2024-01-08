use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum RequestKind {
    Get = 0,
    Set = 1,
    Delete = 2,
    Clear = 3,
    Ping = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum ResponseKind {
    Ok = 0,
    Error = 1,
    Value = 2,
    KeyNotFound = 3,
    Pong = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum ErrorKind {
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
            ResponseKind::Ok => (),
            variant => panic!("got unexpected variant: {:?}", variant),
        }
    }
}
