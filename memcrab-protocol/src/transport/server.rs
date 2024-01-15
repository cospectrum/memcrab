use crate::{
    io::{AsyncReader, AsyncWriter},
    mapping::tokens::{Payload, RequestHeader},
    MemcrabError, ParsingError, Request, Response,
};

#[derive(Debug, Clone)]
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
    pub async fn recv_request(&mut self) -> Result<Request, MemcrabError> {
        let header_chunk = self.stream.read_chunk(RequestHeader::size()).await?;
        let header = self.decode_request_header(&header_chunk)?;

        let payload_chunk = self.stream.read_chunk(header.payload_len()).await?;
        let payload = self.decode_request_payload(header, &payload_chunk)?;

        let req = self.glue(header, payload);
        Ok(req)
    }
    fn glue(&self, header: RequestHeader, payload: Payload) -> Request {
        todo!()
    }
    fn decode_request_header(&self, header_chunk: &[u8]) -> Result<RequestHeader, ParsingError> {
        todo!()
    }
    fn decode_request_payload(
        &self,
        header: RequestHeader,
        payload_chunk: &[u8],
    ) -> Result<Payload, ParsingError> {
        todo!()
    }
}

impl<S> ServerSocket<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub async fn send_response(&mut self, response: &Response) -> Result<(), MemcrabError> {
        let response_bytes = self.encode_response(response);
        self.stream.write_all(&response_bytes).await?;
        Ok(())
    }
    fn encode_response(&self, response: &Response) -> Vec<u8> {
        todo!()
    }
}
