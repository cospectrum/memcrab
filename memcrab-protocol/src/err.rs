use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientSideError {
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("parsing failed")]
    Parsing(#[from] ParsingError),
}

pub type ServerSideError = ClientSideError;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("invalid header")]
    Header,
    #[error("invalid payload")]
    Payload,
}
