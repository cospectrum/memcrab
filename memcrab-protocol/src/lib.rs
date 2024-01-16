mod err;

#[allow(unused)]
mod transport;

#[allow(unused)]
pub(crate) mod mapping;

pub mod io;

use mapping::alias::Version;

pub use err::{ClientSideError, ParsingError, ServerSideError};
pub use transport::{ClientSocket, ErrorResponse, Request, Response, ServerSocket};

pub const PROTOCOL_VERSION: Version = 0;
