use crate::{
    io::{AsyncReader, AsyncWriter},
    ProtocolError,
};

pub struct ServerSocket<S> {
    stream: S,
}

impl<S> ServerSocket<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    #[allow(unused)]
    pub async fn serve_next_request(&mut self) -> Result<(), ProtocolError> {
        let header_size = 0;
        let header_bytes = self.stream.read_chunk(header_size).await?;
        todo!("parse header")
    }
}
