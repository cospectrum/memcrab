use crate::{
    alias::{Expiration, KeyLen, PayloadLen, Version},
    kind::{MsgKind, RequestKind, ResponseKind},
    Msg, ParseError, Request, Response, HEADER_SIZE,
};
use itertools::chain;
use std::{mem::size_of, string::FromUtf8Error};

#[derive(Clone, Debug)]
pub struct Parser;

impl Parser {
    pub fn decode_header(
        &self,
        chunk: &[u8; HEADER_SIZE],
    ) -> Result<(MsgKind, PayloadLen), ParseError> {
        let kind = MsgKind::try_from(chunk[0])?;
        let payload_len = PayloadLen::from_be_bytes(chunk[1..].try_into()?);
        Ok((kind, payload_len))
    }
}

type Payload = Vec<u8>;

impl Parser {
    pub fn decode(&self, kind: MsgKind, payload: Payload) -> Result<Msg, ParseError> {
        Ok(match kind {
            MsgKind::Request(kind) => Msg::Request(self.decode_request(kind, payload)?),
            MsgKind::Response(kind) => Msg::Response(self.decode_response(kind, payload)?),
        })
    }
    fn decode_request(&self, kind: RequestKind, payload: Payload) -> Result<Request, ParseError> {
        use RequestKind as Kind;
        Ok(match kind {
            Kind::Version => {
                let version = Version::from_be_bytes(payload.as_slice().try_into()?);
                Request::Version(version)
            }
            Kind::Ping => Request::Ping,
            Kind::Get => Request::Get(utf8(payload)?),
            Kind::Set => {
                let (klen_bytes, tail) = payload.split_at(size_of::<KeyLen>());
                let (exp_bytes, tail) = tail.split_at(size_of::<Expiration>());

                let klen = KeyLen::from_be_bytes(klen_bytes.try_into()?);
                let expiration = Expiration::from_be_bytes(exp_bytes.try_into()?);

                let (key, value) = tail.split_at(klen as usize);
                let key = utf8(key)?;
                Request::Set {
                    key,
                    value: value.to_vec(),
                    expiration,
                }
            }
            Kind::Clear => Request::Clear,
            Kind::Delete => Request::Delete(utf8(payload)?),
        })
    }
    fn decode_response(
        &self,
        kind: ResponseKind,
        payload: Payload,
    ) -> Result<Response, ParseError> {
        use ResponseKind as Kind;
        Ok(match kind {
            Kind::Ok => Response::Ok,
            Kind::Pong => Response::Pong,
            Kind::KeyNotFound => Response::KeyNotFound,
            Kind::Value => Response::Value(payload),
            Kind::Error => Response::Error(utf8(payload)?),
        })
    }
}

impl Parser {
    // (kind + payload_len) + payload
    pub fn encode(&self, msg: Msg) -> Vec<u8> {
        let (kind, payload) = match msg {
            Msg::Request(req) => {
                let (kind, payload) = self.encode_request(req);
                (MsgKind::Request(kind), payload)
            }
            Msg::Response(resp) => {
                let (kind, payload) = self.encode_response(resp);
                (MsgKind::Response(kind), payload)
            }
        };
        let payload_len = (payload.len() as u64).to_be_bytes();
        chain!([kind.into()], payload_len, payload).collect()
    }

    fn encode_request(&self, req: Request) -> (RequestKind, Payload) {
        match req {
            Request::Version(version) => (RequestKind::Version, version.to_be_bytes().to_vec()),
            Request::Ping => (RequestKind::Ping, vec![]),
            Request::Clear => (RequestKind::Clear, vec![]),
            Request::Get(key) => (RequestKind::Get, key.into()),
            Request::Delete(key) => (RequestKind::Delete, key.into()),
            Request::Set {
                key,
                value,
                expiration,
            } => {
                let key = Vec::from(key);
                let klen = (key.len() as KeyLen).to_be_bytes();
                let exp = expiration.to_be_bytes();

                let payload = chain!(klen, exp, key, value).collect();
                (RequestKind::Set, payload)
            }
        }
    }
    fn encode_response(&self, resp: Response) -> (ResponseKind, Payload) {
        match resp {
            Response::Ok => (ResponseKind::Ok, vec![]),
            Response::Pong => (ResponseKind::Pong, vec![]),
            Response::KeyNotFound => (ResponseKind::KeyNotFound, vec![]),
            Response::Value(val) => (ResponseKind::Value, val),
            Response::Error(emsg) => (ResponseKind::Error, emsg.into()),
        }
    }
}

fn utf8(bytes: impl Into<Vec<u8>>) -> Result<String, FromUtf8Error> {
    String::from_utf8(bytes.into())
}
