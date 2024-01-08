use super::{
    // err::ParsingError,
    // flags::RequestKind,
    // tokens::{Expiration, KeyLen, RequestHeader, Payload, Request, ValueLen},
    tokens::Response,
};
use crate::{
    io::{AsyncReader, AsyncWriter},
    ProtocolError,
};

pub struct ClientParser<S> {
    #[allow(unused)]
    stream: S,
}

impl<S> ClientParser<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    #[allow(unused)]
    pub async fn decode_response(&mut self) -> Result<Response, ProtocolError> {
        todo!()
    }

    #[allow(unused)]
    pub async fn encode_request(&mut self) -> Result<(), ProtocolError> {
        todo!()
    }
}
