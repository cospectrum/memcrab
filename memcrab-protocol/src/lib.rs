mod parser;

#[allow(unused)]
pub(crate) mod mapping;

pub mod io;

use mapping::alias::Version;

pub use parser::{Error, ParseError};
pub use parser::{ErrorResponse, Request, Response, ServerSocket};

pub const PROTOCOL_VERSION: Version = 0;
