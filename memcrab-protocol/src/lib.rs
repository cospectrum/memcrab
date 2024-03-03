mod alias;
mod err;
mod kind;
mod msg;
mod parser;
mod socket;

use parser::Parser;
use std::mem::size_of;

pub use err::{Error, ParseError};
pub use msg::{Msg, Request, Response};
pub use socket::Socket;
pub use tokio::io::{AsyncRead, AsyncWrite};

const HEADER_SIZE: usize = size_of::<u8>() + size_of::<u64>();

#[cfg(test)]
mod tests {
    use crate::HEADER_SIZE;

    #[test]
    fn test_size() {
        assert_eq!(HEADER_SIZE, 9);
    }
}
