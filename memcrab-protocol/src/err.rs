use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io")]
    IO(#[from] std::io::Error),

    #[error("cannot parse message")]
    Parse(#[from] ParseError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid message kind")]
    UnknownMsgKind,

    #[error("malformed string")]
    InvalidString,

    #[error("message is too big")]
    TooBig,
}
