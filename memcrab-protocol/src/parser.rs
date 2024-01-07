use crate::tokens;

pub struct Parser;

#[allow(unused)]
impl Parser {
    pub fn encode_request(&self, req: tokens::Request) -> Vec<u8> {
        todo!()
    }
    pub fn encode_response(&self, resp: tokens::Response) -> Vec<u8> {
        todo!()
    }
    pub fn decode_request(&self, req: &[u8]) -> tokens::Request {
        todo!()
    }
    pub fn decode_response(&self, resp: &[u8]) -> tokens::Response {
        todo!()
    }
}
