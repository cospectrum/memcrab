use crate::{
    io::{AsyncReader, AsyncWriter},
    mapping::{
        flags::ResponseFlag,
        tokens::{ErrorHeader, Payload, ResponseHeader},
    },
    ClientSideError, ErrorResponse, ParsingError, Request, Response,
};

#[derive(Debug, Clone)]
pub struct ClientSocket<S> {
    stream: S,
}

impl<S> ClientSocket<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
    pub async fn make_request(&mut self, request: &Request) -> Result<Response, ClientSideError> {
        let req_bytes = self.encode_request(request);
        self.stream.write_all(&req_bytes).await?;

        let header_chunk = self.stream.read_chunk(ResponseHeader::size()).await?;
        let header = self.decode_response_header(&header_chunk)?;

        let payload_chunk = self.stream.read_chunk(header.payload_len()).await?;
        let payload = self.decode_response_payload(header, &payload_chunk)?;

        let resp = self.glue(header, payload);
        Ok(resp)
    }
    // TODO: maybe return Result to avoid panics
    fn glue(&self, header: ResponseHeader, payload: Payload) -> Response {
        match header {
            ResponseHeader::Ok => Response::Ok,
            ResponseHeader::Pong => Response::Pong,
            ResponseHeader::KeyNotFound => Response::KeyNotFound,
            ResponseHeader::Value { .. } => {
                match payload {
                    Payload::Value(v) => Response::Value(v),
                    _ => unreachable!(), // TODO
                }
            }
            ResponseHeader::Error(err) => {
                let msg = match payload {
                    Payload::ErrMsg(msg) => msg,
                    Payload::Zero => "".to_owned(),
                    _ => panic!("invalid payload"), // TODO
                };
                match err {
                    ErrorHeader::Internal { .. } => Response::Error(ErrorResponse::Internal(msg)),
                    ErrorHeader::Validation { .. } => {
                        Response::Error(ErrorResponse::Validation(msg))
                    }
                }
            }
        }
    }
    fn encode_request(&self, request: &Request) -> Vec<u8> {
        todo!()
    }
    fn decode_response_header(&self, h: &[u8]) -> Result<ResponseHeader, ParsingError> {
        let flag = ResponseFlag::try_from(h[0]).map_err(|_| ParsingError::Header)?;
        match flag {
            ResponseFlag::Pong => Ok(ResponseHeader::Pong),
            ResponseFlag::Ok => Ok(ResponseHeader::Ok),
            ResponseFlag::KeyNotFound => Ok(ResponseHeader::KeyNotFound),
            ResponseFlag::Value => todo!(),
            ResponseFlag::InternalErr => todo!(),
            ResponseFlag::ValidationErr => todo!(),
        }
    }
    fn decode_response_payload(
        &self,
        header: ResponseHeader,
        payload_chunk: &[u8],
    ) -> Result<Payload, ParsingError> {
        todo!()
    }
}
