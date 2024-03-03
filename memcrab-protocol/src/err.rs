use crate::Msg;
use std::{array::TryFromSliceError, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io")]
    IO(#[from] std::io::Error),

    #[error("cannot parse message")]
    Parse(#[from] ParseError),

    #[error("invalid msg")]
    InvalidMsg(Msg),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid message kind")]
    UnknownMsgKind,

    #[error("malformed string")]
    InvalidString(#[from] FromUtf8Error),

    #[error("message is too big")]
    TooBig,

    #[error("conversion from a slice to an array fails")]
    TryFromSlice(#[from] TryFromSliceError),
}
