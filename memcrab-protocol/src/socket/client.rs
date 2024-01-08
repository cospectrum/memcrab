use crate::{
    io::{AsyncReader, AsyncWriter},
    parser::Parser,
    tokens, ProtocolError,
};

pub struct ClientSocket<S> {
    stream: S,
    parser: Parser,
}

impl<S> ClientSocket<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            parser: Parser,
        }
    }

    #[allow(unused)]
    pub async fn make_request(
        &mut self,
        req: tokens::Request,
    ) -> Result<tokens::Response, ProtocolError> {
        let req = self.parser.encode_request(req);
        self.stream.write_all(&req).await?;

        let header_size = 0;
        let resp = self.stream.read_chunk(header_size).await?;
        let resp = self.parser.decode_response(&resp);

        todo!()
    }
}
