use crate::{
    io::{AsyncReader, AsyncWriter},
    Parser, ProtocolError,
};

#[allow(unused)]
pub struct ServerSocket<S> {
    stream: S,
    parser: Parser,
}

impl<S> ServerSocket<S>
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
    pub async fn serve_next_request(&mut self) -> Result<(), ProtocolError> {
        let header_size = 0;
        let header_bytes = self.stream.read_chunk(header_size).await?;
        todo!("parse header")
    }
}
