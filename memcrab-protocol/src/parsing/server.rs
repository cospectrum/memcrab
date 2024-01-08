use crate::{
    io::{AsyncReader, AsyncWriter},
    parsing::tokens,
    ProtocolError,
};

#[allow(unused)]
pub struct ServerParser<S> {
    stream: S,
}

impl<S> ServerParser<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    #[allow(unused)]
    pub async fn decode_request(&mut self) -> Result<(), ProtocolError> {
        let header = self.stream.read_chunk(tokens::RequestHeader::size());
        todo!()
    }

    #[allow(unused)]
    pub async fn encode_response(&mut self) -> Result<(), ProtocolError> {
        todo!()
    }
}
