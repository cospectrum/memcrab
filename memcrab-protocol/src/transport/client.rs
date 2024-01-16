use crate::{
    io::{AsyncReader, AsyncWriter},
    mapping::{
        alias::{ErrMsgLen, Expiration, KeyLen, ValueLen},
        flags::{RequestFlag, ResponseFlag},
        tokens::{ErrorHeader, Payload, RequestHeader, ResponseHeader},
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

        let header_chunk = self.stream.read_chunk(ResponseHeader::SIZE).await?;
        let header = self.decode_response_header(&header_chunk)?;

        let payload_chunk = if header.payload_len() > 0 {
            self.stream.read_chunk(header.payload_len()).await?
        } else {
            vec![]
        };
        let payload = self.decode_response_payload(header, payload_chunk)?;

        let resp = self.construct_response(header, payload);
        Ok(resp)
    }

    fn construct_response(&self, header: ResponseHeader, payload: Payload) -> Response {
        match header {
            ResponseHeader::Ok => Response::Ok,
            ResponseHeader::Pong => Response::Pong,
            ResponseHeader::KeyNotFound => Response::KeyNotFound,
            ResponseHeader::Value { .. } => match payload {
                Payload::Zero => Response::Value(vec![]),
                Payload::Value(v) => Response::Value(v),
                p => panic!("invalid value, payload={:?}", p),
            },
            ResponseHeader::Error(err) => {
                let msg = match payload {
                    Payload::Zero => "".to_owned(),
                    Payload::ErrMsg(msg) => msg,
                    p => panic!("invalid error msg, payload={:?}", p),
                };
                let inner = match err {
                    ErrorHeader::Internal { .. } => ErrorResponse::Internal(msg),
                    ErrorHeader::Validation { .. } => ErrorResponse::Validation(msg),
                };
                Response::Error(inner)
            }
        }
    }
    fn encode_request(&self, request: &Request) -> Vec<u8> {
        let mut bytes = vec![0; RequestHeader::SIZE];
        match request {
            Request::Ping => {
                bytes[0] = RequestFlag::Ping.into();
            }
            Request::Clear => {
                bytes[0] = RequestFlag::Clear.into();
            }
            Request::Version(v) => {
                bytes[0] = RequestFlag::Version.into();
                let [a, b] = v.to_be_bytes();
                bytes[1] = a;
                bytes[2] = b;
            }
            Request::Get(key) => {
                bytes[0] = RequestFlag::Get.into();
                let key = key.as_bytes();
                let klen: KeyLen = key.len().try_into().unwrap();

                for (dst, src) in bytes[1..].iter_mut().zip(klen.to_be_bytes()) {
                    *dst = src;
                }
                bytes.extend_from_slice(key);
            }
            Request::Delete(key) => {
                bytes[0] = RequestFlag::Delete.into();
                let key = key.as_bytes();
                let klen: KeyLen = key.len().try_into().unwrap();

                for (dst, src) in bytes[1..].iter_mut().zip(klen.to_be_bytes()) {
                    *dst = src;
                }
                bytes.extend_from_slice(key);
            }
            Request::Set {
                key,
                value,
                expiration,
            } => {
                bytes[0] = RequestFlag::Set.into();
                let key = key.as_bytes();
                let klen: KeyLen = key.len().try_into().unwrap();
                let vlen: ValueLen = value.len().try_into().unwrap();

                let klen_bytes = klen.to_be_bytes();
                let vlen_bytes = vlen.to_be_bytes();
                let exp_bytes = expiration.to_be_bytes();

                let tail = klen_bytes
                    .iter()
                    .chain(vlen_bytes.iter())
                    .chain(exp_bytes.iter());
                for (dst, &src) in bytes[1..].iter_mut().zip(tail) {
                    *dst = src;
                }
                bytes.extend_from_slice(key);
                bytes.extend_from_slice(value);
            }
        }
        bytes
    }
    fn decode_response_header(&self, header_chunk: &[u8]) -> Result<ResponseHeader, ParsingError> {
        let flag = ResponseFlag::try_from(header_chunk[0]).map_err(|_| ParsingError::Header)?;
        match flag {
            ResponseFlag::Pong => Ok(ResponseHeader::Pong),
            ResponseFlag::Ok => Ok(ResponseHeader::Ok),
            ResponseFlag::KeyNotFound => Ok(ResponseHeader::KeyNotFound),
            ResponseFlag::Value => {
                let vlen_bytes = &header_chunk[1..1 + ResponseHeader::VLEN_SIZE];
                let vlen = KeyLen::from_be_bytes(
                    vlen_bytes
                        .try_into()
                        .expect("vlen_bytes.len() should be equal to VLEN_SIZE"),
                );
                Ok(ResponseHeader::Value { vlen })
            }
            ResponseFlag::InternalErr => {
                let msg_len_bytes = &header_chunk[1..1 + ErrorHeader::MSG_LEN_SIZE];
                let msg_len = ErrMsgLen::from_be_bytes(
                    msg_len_bytes
                        .try_into()
                        .expect("msg_len_bytes.len() should be equal to MSG_LEN_SIZE"),
                );
                let err = ErrorHeader::Internal { len: msg_len };
                Ok(ResponseHeader::Error(err))
            }
            ResponseFlag::ValidationErr => {
                let msg_len_bytes = &header_chunk[1..1 + ErrorHeader::MSG_LEN_SIZE];
                let msg_len = ErrMsgLen::from_be_bytes(
                    msg_len_bytes
                        .try_into()
                        .expect("msg_len_bytes.len() should be equal to MSG_LEN_SIZE"),
                );
                let err = ErrorHeader::Validation { len: msg_len };
                Ok(ResponseHeader::Error(err))
            }
        }
    }
    fn decode_response_payload(
        &self,
        header: ResponseHeader,
        payload_chunk: Vec<u8>,
    ) -> Result<Payload, ParsingError> {
        use ResponseHeader as H;

        match header {
            H::Pong | H::Ok | H::KeyNotFound => Ok(Payload::Zero),
            H::Value { vlen } => {
                assert_eq!(vlen, payload_chunk.len() as u64);
                Ok(Payload::Value(payload_chunk))
            }
            H::Error(inner) => {
                assert_eq!(inner.errmsg_len(), payload_chunk.len() as u64);
                let msg = String::from_utf8(payload_chunk).map_err(|_| ParsingError::Payload)?;
                Ok(Payload::ErrMsg(msg))
            }
        }
    }
}
