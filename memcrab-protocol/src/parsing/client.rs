use crate::{
    io::{AsyncReader, AsyncWriter},
    parsing::tokens,
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
    pub async fn encode_request(
        &mut self,
        req: tokens::Request,
    ) -> Result<tokens::Response, ProtocolError> {
        todo!()
    }

    #[allow(unused)]
    pub async fn decode_response(
        &mut self,
        req: tokens::Request,
    ) -> Result<tokens::Response, ProtocolError> {
        todo!()
    }
}
