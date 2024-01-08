use crate::tokens;
use thiserror::Error;

pub struct Parser;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("invalid header")]
    Header,
    #[error("invalid payload")]
    Payload,
}

#[allow(unused)]
impl Parser {
    pub fn encode_request(&self, req: tokens::Request) -> Vec<u8> {
        todo!()
    }
    pub fn encode_response(&self, resp: tokens::Response) -> Vec<u8> {
        todo!()
    }
    pub fn decode_request(&self, req: &[u8]) -> Result<tokens::Request, ParsingError> {
        todo!()
    }
    pub fn decode_response(&self, resp: &[u8]) -> Result<tokens::Response, ParsingError> {
        todo!()
    }
}
