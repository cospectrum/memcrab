mod err;
mod parsing;

pub mod io;

pub use parsing::ClientParser;
pub use parsing::ServerParser;

pub use err::ProtocolError;

type ProtocolVersion = u16;
pub const PROTOCOL_VERSION: ProtocolVersion = 0;
