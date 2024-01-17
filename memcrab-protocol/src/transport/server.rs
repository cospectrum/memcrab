use crate::{
    io::{AsyncReader, AsyncWriter},
    mapping::{
        alias::{ErrMsgLen, Expiration, KeyLen, ValueLen, Version},
        flags::{RequestKind, ResponseFlag},
        tokens::{Payload, RequestHeader, ResponseHeader},
    },
    ErrorResponse, ParsingError, Request, Response, ServerSideError,
};

#[derive(Debug, Clone)]
pub struct ServerSocket<S> {
    stream: S,
}

pub struct ParseError(String);

impl<S> ServerSocket<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    pub async fn read_request(&mut self) -> Result<Request, ParseError> {
        let buf = self
            .stream
            .read_chunk(RequestHeader::SIZE)
            .await
            .map_err(|e| ParseError("Cannot read data from socket".to_string()))?;

        let kind = RequestKind::try_from(buf[0])
            .map_err(|_| ParseError("Invalid request kind".to_string()))?;
        use RequestKind::*;
        Ok(match kind {
            Version => {
                let version = self.parse_version_header(&buf)?;
                Request::Version(version)
            }
            Ping => Request::Ping,
            Get => {
                let klen = self.parse_get_header(&buf)?;
                let key_bytes = self.stream.read_chunk(klen as usize).await.unwrap();
                let key = Self::parse_utf8(key_bytes)?;

                Request::Get(key)
            }
            Set => {
                let (klen, vlen, expiration) = self.parse_set_header(&buf)?;
                let buf = self.stream.read_chunk(klen as usize).await.unwrap();
                let (key, value) = buf.split_at(klen as usize);
                let (key, value) = (Self::parse_utf8(key.to_vec())?, value.to_vec());
                Request::Set {
                    key,
                    value,
                    expiration,
                }
            }
            Clear => Request::Clear,
            Delete => {
                let klen = self.parse_get_header(&buf)?;
                let key_bytes = self.stream.read_chunk(klen as usize).await.unwrap();
                let key = Self::parse_utf8(key_bytes)?;

                Request::Delete(key)
            }
        })
    }

    fn parse_version_header(&mut self, header: &[u8]) -> Result<Version, ParseError> {
        let version_bytes = &header[..RequestHeader::VERSION_SIZE];
        let version = Version::from_be_bytes(
            version_bytes
                .try_into()
                .expect("version_bytes len != VERSION_SIZE"),
        );
        Ok(version)
    }

    fn parse_get_header(&mut self, header: &[u8]) -> Result<KeyLen, ParseError> {
        let klen_bytes = &header[..RequestHeader::KLEN_SIZE];
        let klen = KeyLen::from_be_bytes(
            klen_bytes
                .try_into()
                .expect("klen_bytes.len() should be equal to KLEN_SIZE"),
        );
        Ok(klen)
    }

    fn parse_set_header(
        &mut self,
        header: &[u8],
    ) -> Result<(KeyLen, ValueLen, Expiration), ParseError> {
        let rest = &header[1..];
        let (klen_bytes, rest) = rest.split_at(RequestHeader::KLEN_SIZE);
        let (vlen_bytes, rest) = rest.split_at(RequestHeader::VLEN_SIZE);
        let expiration_bytes = &rest[..RequestHeader::EXP_SIZE];

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

        Ok((klen, vlen, expiration))
    }

    fn parse_utf8(buf: Vec<u8>) -> Result<String, ParseError> {
        String::from_utf8(buf).map_err(|_| ParseError("Invalid string for key".to_string()))
    }

    pub async fn send_response(&mut self, response: &Response) -> Result<(), ServerSideError> {
        let response_bytes = self.encode_response(response);
        self.stream.write_all(&response_bytes).await?;
        Ok(())
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
