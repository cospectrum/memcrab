use crate::{err::Error, Msg, Parser, HEADER_SIZE};
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Wraps a stream (typically TCPStream) for receiving/sending framed messages according to memcrab
/// protocol. It is used for both client and server implementations.
#[derive(Debug, Clone)]
pub struct Socket<S> {
    stream: S,
    parser: Parser,
}

impl<S> Socket<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            parser: Parser,
        }
    }
}

impl<S> Socket<S>
where
    S: AsyncWrite + Unpin,
{
    /// Encode a message and write it to the socket.
    pub async fn send(&mut self, msg: Msg) -> Result<(), Error> {
        let bytes = self.parser.encode(msg);
        self.stream.write_all(&bytes).await?;
        Ok(())
    }
}

impl<S> Socket<S>
where
    S: AsyncRead + Unpin,
{
    /// Wait for a complete message from socket and parse it.
    pub async fn recv(&mut self) -> Result<Msg, Error> {
        let mut header = [0; HEADER_SIZE];
        self.stream.read_exact(&mut header).await?;
        let (kind, payload_len) = self.parser.decode_header(&header)?;

        let payload = if payload_len > 0 {
            read_chunk_exact(&mut self.stream, payload_len as usize).await?
        } else {
            vec![]
        };
        let msg = self.parser.decode(kind, payload)?;
        Ok(msg)
    }
}

async fn read_chunk_exact<S: AsyncRead + Unpin>(
    stream: &mut S,
    size: usize,
) -> Result<Vec<u8>, io::Error> {
    let mut buf = vec![0; size];
    let n = stream.read_exact(&mut buf).await?;
    assert_eq!(n, buf.len());
    Ok(buf)
}

// test in submodule so we can access private stream
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        kind::{MsgKind, RequestKind, ResponseKind},
        Request, Response,
    };
    use itertools::chain;
    use tokio_test::io::Builder;

    #[allow(unused)]
    async fn assert_parsed(data: impl AsRef<[u8]>, msg: Msg) {
        let mock = Builder::new().read(data.as_ref()).build();
        let mut socket = Socket::new(mock);
        let parsed = socket.recv().await.unwrap();
        assert_eq!(parsed, msg);
    }

    #[tokio::test]
    async fn test_socket() {
        let zero_u64_bytes = &0u64.to_be_bytes();

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
