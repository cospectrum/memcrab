use std::{mem::size_of, string::FromUtf8Error};

use itertools::chain;

use crate::{
    alias::{Expiration, KeyLen, PayloadLen, Version},
    kind::MsgKind,
    Msg, ParseError, Request, Response, HEADER_SIZE,
};

#[derive(Clone, Debug)]
pub struct Parser;

impl Parser {
    pub fn decode_header(
        &self,
        chunk: &[u8; HEADER_SIZE],
    ) -> Result<(MsgKind, PayloadLen), ParseError> {
        let kind = MsgKind::try_from(chunk[0]).map_err(|_| ParseError::UnknownMsgKind)?;
        let payload_len = PayloadLen::from_be_bytes(chunk[1..].try_into()?);
        Ok((kind, payload_len))
    }
}

type Payload = Vec<u8>;

impl Parser {
    pub fn decode(&self, kind: MsgKind, payload: Payload) -> Result<Msg, ParseError> {
        use MsgKind as Kind;
        Ok(match kind {
            Kind::VersionRequest => {
                let version = Version::from_be_bytes(payload.as_slice().try_into()?);
                Msg::Request(Request::Version(version))
            }
            Kind::PingRequest => Msg::Request(Request::Ping),
            Kind::GetRequest => Msg::Request(Request::Get(utf8(payload)?)),
            Kind::SetRequest => {
                let (klen_bytes, tail) = payload.split_at(size_of::<KeyLen>());
                let (exp_bytes, tail) = tail.split_at(size_of::<Expiration>());

                let klen = KeyLen::from_be_bytes(klen_bytes.try_into()?);
                let expiration = Expiration::from_be_bytes(exp_bytes.try_into()?);

                let (key, value) = tail.split_at(klen as usize);
                let key = utf8(key)?;
                Msg::Request(Request::Set {
                    key,
                    value: value.to_vec(),
                    expiration,
                })
            }
            Kind::ClearRequest => Msg::Request(Request::Clear),
            Kind::DeleteRequest => Msg::Request(Request::Delete(utf8(payload)?)),

            Kind::OkResponse => Msg::Response(Response::Ok),
            Kind::PongResponse => Msg::Response(Response::Pong),
            Kind::KeyNotFoundResponse => Msg::Response(Response::KeyNotFound),
            Kind::ValueResponse => Msg::Response(Response::Value(payload)),
            Kind::ErrorResponse => Msg::Response(Response::Error(utf8(payload)?)),
        })
    }
}

impl Parser {
    // (kind + payload_len) + payload
    pub fn encode(&self, msg: Msg) -> Vec<u8> {
        let (kind, payload) = match msg {
            Msg::Request(req) => self.encode_request(req),
            Msg::Response(resp) => self.encode_response(resp),
        };
        let payload_len = (payload.len() as u64).to_be_bytes();
        chain!([kind.into()], payload_len, payload).collect()
    }

    fn encode_request(&self, req: Request) -> (MsgKind, Payload) {
        match req {
            Request::Version(version) => (MsgKind::VersionRequest, version.to_be_bytes().to_vec()),
            Request::Ping => (MsgKind::PingRequest, vec![]),
            Request::Clear => (MsgKind::ClearRequest, vec![]),
            Request::Get(key) => (MsgKind::GetRequest, key.into()),
            Request::Delete(key) => (MsgKind::DeleteRequest, key.into()),
            Request::Set {
                key,
                value,
                expiration,
            } => {
                let key = Vec::from(key);
                let klen = (key.len() as KeyLen).to_be_bytes();
                let exp = expiration.to_be_bytes();

                let payload = chain!(klen, exp, key, value).collect();
                (MsgKind::SetRequest, payload)
            }
        }
    }
    fn encode_response(&self, resp: Response) -> (MsgKind, Payload) {
        match resp {
            Response::Ok => (MsgKind::OkResponse, vec![]),
            Response::Pong => (MsgKind::PongResponse, vec![]),
            Response::KeyNotFound => (MsgKind::KeyNotFoundResponse, vec![]),
            Response::Value(val) => (MsgKind::ValueResponse, val),
            Response::Error(emsg) => (MsgKind::ErrorResponse, emsg.into()),
        }
    }
}

fn utf8(bytes: impl Into<Vec<u8>>) -> Result<String, FromUtf8Error> {
    String::from_utf8(bytes.into())
}
