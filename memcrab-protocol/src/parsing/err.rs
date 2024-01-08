use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("invalid header")]
    Header,
    #[error("invalid payload")]
    Payload,
}
