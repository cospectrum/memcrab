use memcrab_protocol::Error as ProtocolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerSideError {
    #[error("protocol error")]
    Protocol(#[from] ProtocolError),

    #[error("invalid msg")]
    InvalidMsg,
}
