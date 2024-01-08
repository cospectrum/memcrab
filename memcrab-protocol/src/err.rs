use crate::ParsingError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("failed to fill buffer")]
    IncompleteRead { expected_size: usize, buf: Vec<u8> },
    #[error("parser failed")]
    Parsing(#[from] ParsingError),
}
