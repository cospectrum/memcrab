use super::{
    err::ParsingError,
    flags::RequestKind,
    tokens::{Expiration, KeyLen, Payload, Request, RequestHeader, ValueLen},
};
use crate::{
    io::{AsyncReader, AsyncWriter},
    ProtocolError,
};

#[allow(unused)]
pub struct ServerParser<S> {
    stream: S,
}

impl<S> ServerParser<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    #[allow(unused)]
    pub async fn decode_header(&mut self) -> Result<RequestHeader, ProtocolError> {
        let mut header = self.stream.read_chunk(RequestHeader::size()).await?;
        let kind = header[0]
            .try_into()
            .map_err(|e| ProtocolError::Parsing(ParsingError::Header))?;

        // this is horrible

        let k = 1..RequestHeader::klen_size() + 1;
        let v = k.end + 1..RequestHeader::vlen_size();
        let e = v.end + 1..RequestHeader::expiration_size();

        let klen = KeyLen::from_be_bytes(header[k].try_into().unwrap());
        let vlen = ValueLen::from_be_bytes(header[v].try_into().unwrap());
        let expiration = Expiration::from_be_bytes(header[e].try_into().unwrap());

        Ok(match kind {
            RequestKind::Get => RequestHeader::Get { klen },
            RequestKind::Set => RequestHeader::Set {
                klen,
                vlen,
                expiration,
            },
            RequestKind::Delete => RequestHeader::Delete { klen },
            RequestKind::Clear => RequestHeader::Clear {},
            RequestKind::Ping => RequestHeader::Ping {},
        })
    }

    #[allow(unused)]
    pub async fn decode_request(&mut self) -> Result<Request, ProtocolError> {
        let header = self.decode_header().await?;
        Ok(match header {
            RequestHeader::Get { klen } => {
                let key_bytes = self
                    .stream
                    .read_chunk(klen as usize)
                    .await
                    .map_err(|_| ProtocolError::Parsing(ParsingError::Payload))?;
                let key = String::from_utf8(key_bytes)
                    .map_err(|_| ProtocolError::Parsing(ParsingError::Payload))?;
                let payload = Payload::Key { key };
                Request { header, payload }
            }
            RequestHeader::Set {
                klen,
                vlen,
                expiration,
            } => {
                todo!();
            }
            RequestHeader::Delete { klen } => {
                todo!();
            }
            RequestHeader::Clear => {
                todo!();
            }
            RequestHeader::Ping => {
                todo!();
            }
        })
    }

    #[allow(unused)]
    pub async fn encode_response(&mut self) -> Result<(), ProtocolError> {
        todo!()
    }
}
