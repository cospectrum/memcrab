use crate::{err::Error, AsyncReader, AsyncWriter, Msg, Parser, HEADER_SIZE};

/// Wraps a stream (typically TCPStream) for receiving/sending framed messages according to memcrab
/// protocol. It is used for both client and server implementations.
#[derive(Debug, Clone)]
pub struct Socket<S> {
    stream: S,
    parser: Parser,
}

impl<S> Socket<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            parser: Parser,
        }
    }
    /// Wait for a complete message from socket and parse it.
    pub async fn receive(&mut self) -> Result<Msg, Error> {
        let mut header = [0; HEADER_SIZE];
        self.stream.read_exact(&mut header).await?;
        let (kind, payload_len) = self.parser.decode_header(&header)?;

        let payload = if payload_len > 0 {
            self.stream.read_chunk(payload_len as usize).await?
        } else {
            vec![]
        };
        let msg = self.parser.decode(kind, payload)?;
        Ok(msg)
    }
    /// Encode a message and write it to the socket.
    pub async fn send(&mut self, msg: Msg) -> Result<(), Error> {
        let bytes = self.parser.encode(msg);
        self.stream.write_all(&bytes).await?;
        Ok(())
    }
}

// test in submodule so we can access private stream
#[cfg(test)]
mod test {
    use itertools::chain;

    use super::*;
    use crate::{
        kind::{MsgKind, RequestKind, ResponseKind},
        Request, Response,
    };
    use std::collections::VecDeque;

    struct MockStream {
        read_data: VecDeque<u8>,
        wrote_data: Vec<u8>,
    }

    #[async_trait::async_trait]
    impl AsyncReader for MockStream {
        async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
            use std::io::{Error, ErrorKind};

            for (at, byte) in buf.iter_mut().enumerate() {
                if self.read_data.is_empty() {
                    let msg = format!(
                        "read_exact: cannot completely fill buffer with len={}, stopped at {}",
                        buf.len(),
                        at,
                    );
                    return Err(Error::new(ErrorKind::UnexpectedEof, msg));
                }
                *byte = self.read_data.pop_front().unwrap();
            }
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl AsyncWriter for MockStream {
        async fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
            self.wrote_data = Vec::from(buf);
            Ok(())
        }
    }

    #[allow(unused)]
    async fn assert_parsed(data: Vec<u8>, msg: Msg) {
        let mut socket = Socket::new(MockStream {
            read_data: vec![].into(),
            wrote_data: vec![],
        });
        socket.stream.read_data = data.into();
        let parsed = socket.receive().await;
        assert_eq!(parsed.expect("error while parsing"), msg);
    }
    #[tokio::test]
    async fn test_socket() {
        let zero_u64_bytes = &0u64.to_be_bytes();
        // TODO: divide test cases to functions so it does not stop at first fail + reduce
        // boilerplate somehow
        // TODO!: tests for encoding

        let mut data = vec![MsgKind::Request(RequestKind::Version).into()];
        data.extend(2u64.to_be_bytes());
        data.extend([0, 1]);
        assert_parsed(data, Msg::Request(Request::Version(1))).await;

        let mut data = vec![MsgKind::Request(RequestKind::Ping).into()];
        data.extend(zero_u64_bytes);
        assert_parsed(data, Msg::Request(Request::Ping)).await;

        let mut data = vec![MsgKind::Request(RequestKind::Get).into()];
        data.extend(2u64.to_be_bytes()); // rest of header
        data.extend([97, 98]); // utf8 encoded key
        assert_parsed(data, Msg::Request(Request::Get("ab".to_owned()))).await;

        let mut data = vec![MsgKind::Request(RequestKind::Set).into()];
        let klen = [0, 0, 0, 0, 0, 0, 0, 2];
        let exp = [0, 0, 1, 0];
        let key = [97, 98]; // utf8 encoded key
        let val = [1, 2, 3];
        let payload: Vec<u8> = chain!(klen, exp, key, val).collect();
        let payload_len = (payload.len() as u64).to_be_bytes();
        data.extend(payload_len);
        data.extend(payload);
        assert_parsed(
            data,
            Msg::Request(Request::Set {
                key: "ab".to_owned(),
                value: vec![1, 2, 3],
                expiration: 256,
            }),
        )
        .await;

        let mut data = vec![MsgKind::Request(RequestKind::Delete).into()];
        data.extend(2u64.to_be_bytes()); // rest of header
        data.extend(vec![97, 98]); // utf8 encoded key
        assert_parsed(data, Msg::Request(Request::Delete("ab".to_owned()))).await;

        let mut data = vec![MsgKind::Request(RequestKind::Clear).into()];
        data.extend(zero_u64_bytes);
        assert_parsed(data, Msg::Request(Request::Clear)).await;

        let mut data = vec![MsgKind::Response(ResponseKind::Pong).into()];
        data.extend(zero_u64_bytes);
        assert_parsed(data, Msg::Response(Response::Pong)).await;

        let mut data = vec![MsgKind::Response(ResponseKind::Ok).into()];
        data.extend(zero_u64_bytes);
        assert_parsed(data, Msg::Response(Response::Ok)).await;

        let mut data = vec![MsgKind::Response(ResponseKind::Value).into()];
        data.extend(4u64.to_be_bytes());
        data.extend(vec![1, 2, 3, 4]); // msg
        assert_parsed(data, Msg::Response(Response::Value(vec![1, 2, 3, 4]))).await;

        let mut data = vec![MsgKind::Response(ResponseKind::KeyNotFound).into()];
        data.extend(zero_u64_bytes);
        assert_parsed(data, Msg::Response(Response::KeyNotFound)).await;

        let mut data = vec![MsgKind::Response(ResponseKind::Error).into()];
        data.extend(3u64.to_be_bytes()); // payload len
        data.extend(vec![101, 114, 114]); // msg
        assert_parsed(data, Msg::Response(Response::Error("err".to_owned()))).await;
    }
}
