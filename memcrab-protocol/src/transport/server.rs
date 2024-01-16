use crate::{
    io::{AsyncReader, AsyncWriter},
    mapping::{
        alias::{ErrMsgLen, Expiration, KeyLen, ValueLen, Version},
        flags::{RequestFlag, ResponseFlag},
        tokens::{Payload, RequestHeader, ResponseHeader},
    },
    ErrorResponse, ParsingError, Request, Response, ServerSideError,
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
    pub async fn recv_request(&mut self) -> Result<Request, ServerSideError> {
        let header_chunk = self.stream.read_chunk(RequestHeader::SIZE).await?;
        let header = self.decode_request_header(&header_chunk)?;

        let payload_chunk = if header.payload_len() > 0 {
            self.stream.read_chunk(header.payload_len()).await?
        } else {
            vec![]
        };
        let payload = self.decode_request_payload(header, payload_chunk)?;

        let req = self.construct_request(header, payload);
        Ok(req)
    }
    pub async fn send_response(&mut self, response: &Response) -> Result<(), ServerSideError> {
        let response_bytes = self.encode_response(response);
        self.stream.write_all(&response_bytes).await?;
        Ok(())
    }

    fn decode_request_header(&self, header_chunk: &[u8]) -> Result<RequestHeader, ParsingError> {
        let flag = RequestFlag::try_from(header_chunk[0]).map_err(|_| ParsingError::Header)?;
        match flag {
            RequestFlag::Version => {
                let version_bytes = &header_chunk[1..1 + RequestHeader::VERSION_SIZE];
                let version = Version::from_be_bytes(
                    version_bytes
                        .try_into()
                        .expect("version_bytes.len() should be equal to VERSION_SIZE"),
                );
                Ok(RequestHeader::Version(version))
            }
            RequestFlag::Ping => Ok(RequestHeader::Ping),
            RequestFlag::Get => {
                let klen_bytes = &header_chunk[1..1 + RequestHeader::KLEN_SIZE];
                let klen = KeyLen::from_be_bytes(
                    klen_bytes
                        .try_into()
                        .expect("klen_bytes.len() should be equal to KLEN_SIZE"),
                );
                Ok(RequestHeader::Get { klen })
            }
            RequestFlag::Set => {
                let tail = &header_chunk[1..];
                let (klen_bytes, tail) = tail.split_at(RequestHeader::KLEN_SIZE);
                let (vlen_bytes, tail) = tail.split_at(RequestHeader::VLEN_SIZE);
                let expiration_bytes = &tail[..RequestHeader::EXP_SIZE];

                let klen = KeyLen::from_be_bytes(
                    klen_bytes
                        .try_into()
                        .expect("klen_bytes.len() should be equal to KLEN_SIZE"),
                );
                let vlen = ValueLen::from_be_bytes(
                    vlen_bytes
                        .try_into()
                        .expect("vlen_bytes.len() should be equal to VLEN_SIZE"),
                );
                let expiration = Expiration::from_be_bytes(
                    expiration_bytes
                        .try_into()
                        .expect("expiration_bytes.len() should be equal to EXP_SIZE"),
                );
                Ok(RequestHeader::Set {
                    klen,
                    vlen,
                    expiration,
                })
            }
            RequestFlag::Clear => Ok(RequestHeader::Clear),
            RequestFlag::Delete => {
                let klen_bytes = &header_chunk[1..1 + RequestHeader::KLEN_SIZE];
                let klen = KeyLen::from_be_bytes(
                    klen_bytes
                        .try_into()
                        .expect("klen_bytes.len() should be equal to KLEN_SIZE"),
                );
                Ok(RequestHeader::Delete { klen })
            }
        }
    }
    fn decode_request_payload(
        &self,
        header: RequestHeader,
        payload_chunk: Vec<u8>,
    ) -> Result<Payload, ParsingError> {
        match header {
            RequestHeader::Ping => Ok(Payload::Zero),
            RequestHeader::Version(v) => Ok(Payload::Zero),
            RequestHeader::Delete { klen } => {
                assert_eq!(klen, payload_chunk.len() as u64);
                let key = String::from_utf8(payload_chunk).map_err(|_| ParsingError::Payload)?;
                Ok(Payload::Key(key))
            }
            RequestHeader::Clear => Ok(Payload::Zero),
            RequestHeader::Get { klen } => {
                assert_eq!(klen, payload_chunk.len() as u64);
                let key = String::from_utf8(payload_chunk).map_err(|_| ParsingError::Payload)?;
                Ok(Payload::Key(key))
            }
            RequestHeader::Set {
                klen,
                vlen,
                expiration,
            } => {
                let (head, tail) = payload_chunk.split_at(klen.try_into().unwrap());
                let key = String::from_utf8(head.to_vec()).map_err(|_| ParsingError::Payload)?;
                let value = tail.to_vec();
                Ok(Payload::Pair { key, value })
            }
        }
    }
    fn construct_request(&self, header: RequestHeader, payload: Payload) -> Request {
        use RequestHeader as H;

        match (header, payload) {
            (H::Ping, Payload::Zero) => Request::Ping,
            (H::Version(v), Payload::Zero) => Request::Version(v),
            (H::Delete { .. }, Payload::Key(key)) => Request::Delete(key),
            (H::Clear, Payload::Zero) => Request::Clear,
            (H::Get { .. }, Payload::Key(key)) => Request::Get(key),
            (H::Set { expiration, .. }, Payload::Pair { key, value }) => Request::Set {
                key,
                value,
                expiration,
            },
            tuple => panic!("invalid (header, payload): {:?}", tuple),
        }
    }
    fn encode_response(&self, response: &Response) -> Vec<u8> {
        let mut bytes = vec![0; ResponseHeader::SIZE];

        match response {
            Response::Pong => {
                bytes[0] = ResponseFlag::Pong.into();
            }
            Response::Ok => {
                bytes[0] = ResponseFlag::Ok.into();
            }
            Response::KeyNotFound => {
                bytes[0] = ResponseFlag::KeyNotFound.into();
            }
            Response::Value(value) => {
                bytes[0] = ResponseFlag::Value.into();
                let vlen: ValueLen = value.len().try_into().unwrap();
                for (dst, src) in bytes[1..].iter_mut().zip(vlen.to_be_bytes()) {
                    *dst = src;
                }
                bytes.extend_from_slice(value);
            }
            Response::Error(err) => {
                match err {
                    ErrorResponse::Internal(msg) => {
                        bytes[0] = ResponseFlag::InternalErr.into();
                        let msg = msg.as_bytes();
                        let msg_len: ErrMsgLen = msg.len().try_into().unwrap();
                        for (dst, src) in bytes[1..].iter_mut().zip(msg_len.to_be_bytes()) {
                            *dst = src;
                        }
                        bytes.extend_from_slice(msg);
                    }
                    ErrorResponse::Validation(msg) => {
                        bytes[0] = ResponseFlag::ValidationErr.into();
                        let msg = msg.as_bytes();
                        let msg_len: ErrMsgLen = msg.len().try_into().unwrap();
                        for (dst, src) in bytes[1..].iter_mut().zip(msg_len.to_be_bytes()) {
                            *dst = src;
                        }
                        bytes.extend_from_slice(msg);
                    }
                };
            }
        }
        bytes
    }
}
