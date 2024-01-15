mod err;

#[allow(unused)]
mod transport;

#[allow(unused)]
pub(crate) mod mapping;

pub mod io;

pub use err::{ErrorResponse, MemcrabError, ParsingError};
pub use transport::{ClientSocket, Request, Response, ServerSocket};

type ProtocolVersion = u16;

pub const PROTOCOL_VERSION: ProtocolVersion = 0;
