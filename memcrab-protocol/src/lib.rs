mod alias;
mod err;
mod io;
mod kind;
mod msg;
mod parser;
mod socket;

use std::mem::size_of;

use alias::Version;
use parser::Parser;

pub use err::{Error, ParseError};
pub use io::{AsyncReader, AsyncWriter};
pub use msg::{Msg, Request, Response};
pub use socket::Socket;

const HEADER_SIZE: usize = size_of::<u8>() + size_of::<u64>();
pub const VERSION: Version = 0;

#[cfg(test)]
mod tests {
    use crate::HEADER_SIZE;

    #[test]
    fn test_size() {
        assert_eq!(HEADER_SIZE, 9);
    }
}
