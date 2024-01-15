use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemcrabError {
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("parsing failed")]
    Parsing(#[from] ParsingError),
    #[error("response error")]
    ErrorResponse(#[from] ErrorResponse),
}

#[derive(Error, Debug, Clone)]
pub enum ErrorResponse {
    #[error("validation error")]
    Validation(String),
    #[error("internal error")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("invalid header")]
    Header,
    #[error("invalid payload")]
    Payload,
}
