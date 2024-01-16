use crate::{
    io::{AsyncReader, AsyncWriter},
    mapping::{
        flags::RequestFlag,
        tokens::{Expiration, KeyLen, Payload, RequestHeader, ValueLen, Version},
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

        let payload_chunk = self.stream.read_chunk(header.payload_len()).await?;
        let payload = self.decode_request_payload(header, &payload_chunk)?;

        let req = self.construct_request(header, payload);
        Ok(req)
    }
    fn decode_request_header(&self, header_chunk: &[u8]) -> Result<RequestHeader, ParsingError> {
        let flag = RequestFlag::try_from(header_chunk[0]).map_err(|_| ParsingError::Header)?;
        match flag {
            RequestFlag::Version => {
                let version_bytes = &header_chunk[..RequestHeader::VERSION_SIZE];
                let version = Version::from_be_bytes(
                    version_bytes
                        .try_into()
                        .expect("version_bytes len != VERSION_SIZE"),
                );
                Ok(RequestHeader::Version(version))
            }
            RequestFlag::Ping => Ok(RequestHeader::Ping),
            RequestFlag::Get => {
                let klen_bytes = &header_chunk[..RequestHeader::KLEN_SIZE];
                let klen = KeyLen::from_be_bytes(
                    klen_bytes.try_into().expect("klen_bytes len != KLEN_SIZE"),
                );
                Ok(RequestHeader::Get { klen })
            }
            RequestFlag::Set => {
                let mut start = 0;
                let klen_bytes = &header_chunk[..RequestHeader::KLEN_SIZE];
                start += RequestHeader::KLEN_SIZE;

                let vlen_bytes = &header_chunk[start..start + RequestHeader::VLEN_SIZE];
                start += RequestHeader::VLEN_SIZE;

                let expiration_bytes = &header_chunk[start..start + RequestHeader::EXP_SIZE];

                let klen = KeyLen::from_be_bytes(
                    klen_bytes.try_into().expect("klen_bytes len != KLEN_SIZE"),
                );
                let vlen = ValueLen::from_be_bytes(
                    vlen_bytes.try_into().expect("vlen_bytes len != VLEN_SIZE"),
                );
                let expiration = Expiration::from_be_bytes(
                    expiration_bytes
                        .try_into()
                        .expect("expiration_bytes len != EXP_SIZE"),
                );
                Ok(RequestHeader::Set {
                    klen,
                    vlen,
                    expiration,
                })
            }
            RequestFlag::Clear => Ok(RequestHeader::Clear),
            RequestFlag::Delete => {
                let klen_bytes = &header_chunk[..RequestHeader::KLEN_SIZE];
                let klen = KeyLen::from_be_bytes(
                    klen_bytes.try_into().expect("klen_bytes len != KLEN_SIZE"),
                );
                Ok(RequestHeader::Delete { klen })
            }
        }
    }
    fn decode_request_payload(
        &self,
        header: RequestHeader,
        payload_chunk: &[u8],
    ) -> Result<Payload, ParsingError> {
        match header {
            RequestHeader::Ping => Ok(Payload::Zero),
            RequestHeader::Version(v) => Ok(Payload::Zero),
            RequestHeader::Delete { klen } => {
                let key_bytes = &payload_chunk[..klen as usize];
                let key =
                    String::from_utf8(key_bytes.to_vec()).map_err(|_| ParsingError::Payload)?;
                Ok(Payload::Key(key))
            }
            RequestHeader::Clear => Ok(Payload::Zero),
            RequestHeader::Get { klen } => {
                let key_bytes = &payload_chunk[..klen as usize];
                let key =
                    String::from_utf8(key_bytes.to_vec()).map_err(|_| ParsingError::Payload)?;
                Ok(Payload::Key(key))
            }
            RequestHeader::Set {
                klen,
                vlen,
                expiration,
            } => {
                let key_bytes = &payload_chunk[..klen as usize];
                let key =
                    String::from_utf8(key_bytes.to_vec()).map_err(|_| ParsingError::Payload)?;
                let value = Vec::from(&payload_chunk[klen as usize..(klen + vlen) as usize]);
                Ok(Payload::Pair { key, value })
            }
        }
    }
    fn construct_request(&self, header: RequestHeader, payload: Payload) -> Request {
        match (header, payload) {
            (RequestHeader::Ping, Payload::Zero) => todo!(),
            (RequestHeader::Version(v), Payload::Zero) => todo!(),
            (RequestHeader::Delete { .. }, Payload::Key(key)) => todo!(),
            (RequestHeader::Clear, Payload::Zero) => todo!(),
            (RequestHeader::Get { .. }, Payload::Key(key)) => todo!(),
            (RequestHeader::Set { expiration, .. }, Payload::Pair { key, value }) => todo!(),
            (_, _) => todo!(),
        }
    }
    pub async fn send_response(&mut self, response: &Response) -> Result<(), ServerSideError> {
        let response_bytes = self.encode_response(response);
        self.stream.write_all(&response_bytes).await?;
        Ok(())
    }
    fn encode_response(&self, response: &Response) -> Vec<u8> {
        todo!()
    }
}
