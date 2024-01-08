mod err;

mod parser;
mod socket;

pub mod io;
#[allow(dead_code)]
pub mod tokens;

pub use err::ProtocolError;
pub use socket::{ClientSocket, ServerSocket};

type ProtocolVersion = u16;
pub const PROTOCOL_VERSION: ProtocolVersion = 0;
