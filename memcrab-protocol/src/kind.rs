use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgKind {
    Request(RequestKind),
    Response(ResponseKind),
}

impl From<MsgKind> for u8 {
    fn from(kind: MsgKind) -> Self {
        match kind {
            MsgKind::Request(x) => x.into(),
            MsgKind::Response(x) => x.into(),
        }
    }
}

impl TryFrom<u8> for MsgKind {
    type Error = ParseError;
    fn try_from(flag: u8) -> Result<Self, Self::Error> {
        if flag < 128 {
            let inner = RequestKind::try_from(flag).map_err(|_| ParseError::UnknownMsgKind)?;
            Ok(MsgKind::Request(inner))
        } else {
            let inner = ResponseKind::try_from(flag).map_err(|_| ParseError::UnknownMsgKind)?;
            Ok(MsgKind::Response(inner))
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq)]
pub enum RequestKind {
    Version = 0,
    Ping = 1,
    Get = 2,
    Set = 3,
    Delete = 4,
    Clear = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq)]
pub enum ResponseKind {
    Pong = 128,
    Ok = 129,
    Value = 130,
    KeyNotFound = 131,

    Error = 255,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_range() {
        (128..=255)
            .map(|x| u8::try_from(x).unwrap())
            .map(RequestKind::try_from)
            .for_each(|res| {
                assert!(res.is_err());
            });
    }
    #[test]
    fn test_response_range() {
        (0..=127)
            .map(|x| u8::try_from(x).unwrap())
            .map(ResponseKind::try_from)
            .for_each(|res| {
                assert!(res.is_err());
            });
    }
    #[test]
    fn test_response() {
        let pong = 128;
        assert_eq!(ResponseKind::Pong, ResponseKind::try_from(pong).unwrap());
        assert_eq!(
            MsgKind::Response(ResponseKind::Pong),
            MsgKind::try_from(pong).unwrap()
        );
    }
}
