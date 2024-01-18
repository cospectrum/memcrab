use crate::{
    alias::{Expiration, KeyLen, ValueLen, Version},
    err::{Error, ParseError},
    kind::MessageKind,
    message::{Message, Request, Response},
    sizes,
    stream::{AsyncReader, AsyncWriter},
};

/// Wraps a stream (typically TCPStream) for receiving/sending framed messages according to memcrab
/// protocol. It is used for both client and server implementations.
#[derive(Debug, Clone)]
pub struct Socket<S> {
    stream: S,
}

impl<S> Socket<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    /// Wait for a complete message from socket and parse it.
    pub async fn receive(&mut self) -> Result<Message, Error> {
        let header = self.stream.read_chunk(sizes::MAX_HEADER_SIZE).await?;

        let kind = MessageKind::try_from(header[0]).map_err(|_| ParseError::UnknownKind)?;
        use MessageKind as Kind;
        Ok(match kind {
            Kind::VersionRequest => {
                let version = self.parse_version_header(&header[1..])?;
                Message::Request(Request::Version(version))
            }
            Kind::PingRequest => Message::Request(Request::Ping),
            Kind::GetRequest => {
                let klen = self.parse_klen_header(&header[1..])?;
                let key_bytes = self.stream.read_chunk(klen as usize).await?;
                let key = Self::parse_utf8(key_bytes)?;
                Message::Request(Request::Get(key))
            }
            Kind::SetRequest => {
                let (klen, vlen, expiration) = self.parse_klen_vlen_exp_header(&header[1..])?;
                let buf = self
                    .stream
                    .read_chunk(klen as usize + vlen as usize)
                    .await?;
                let (key, value) = buf.split_at(klen as usize);
                let (key, value) = (Self::parse_utf8(key.to_vec())?, value.to_vec());
                Message::Request(Request::Set {
                    key,
                    value,
                    expiration,
                })
            }
            Kind::ClearRequest => Message::Request(Request::Clear),
            Kind::DeleteRequest => {
                let klen = self.parse_klen_header(&header[1..])?;
                let key_bytes = self.stream.read_chunk(klen as usize).await.unwrap();
                let key = Self::parse_utf8(key_bytes)?;
                Message::Request(Request::Delete(key))
            }
            Kind::OkResponse => Message::Response(Response::Ok),
            Kind::ErrorResponse => {
                // Error is parsed as Value with message as payload.
                let vlen = self.parse_vlen_header(&header[1..])?;
                let buf = self.stream.read_chunk(vlen as usize).await?;
                let msg = Self::parse_utf8(buf)?;
                Message::Response(Response::Error(msg))
            }
            Kind::PongResponse => Message::Response(Response::Pong),
            Kind::ValueResponse => {
                let vlen = self.parse_vlen_header(&header[1..])?;
                let buf = self.stream.read_chunk(vlen as usize).await?;
                Message::Response(Response::Value(buf))
            }
            Kind::KeyNotFoundResponse => Message::Response(Response::KeyNotFound),
        })
    }

    /// Encode a message and write it to the socket.
    pub async fn send(&mut self, msg: Message) -> Result<(), Error> {
        let mut bytes = Vec::<u8>::new();

        match msg {
            Message::Request(Request::Version(version)) => {
                bytes.push(MessageKind::VersionRequest.into());
                bytes.extend_from_slice(&version.to_be_bytes());
                todo!();
            }
            Message::Request(Request::Get(key)) => {
                bytes.push(MessageKind::GetRequest.into());
                bytes.extend_from_slice(&key.len().to_be_bytes());
                bytes.extend_from_slice(key.as_bytes());
            }
            Message::Request(Request::Clear) => {
                bytes[0] = MessageKind::ClearRequest.into();
            }
            Message::Request(Request::Ping) => {
                bytes.push(MessageKind::PingRequest.into());
            }
            Message::Request(Request::Set {
                key,
                mut value,
                expiration,
            }) => {
                bytes.push(MessageKind::SetRequest.into());
                bytes.extend_from_slice(&key.len().to_be_bytes());
                bytes.extend_from_slice(&value.len().to_be_bytes());
                bytes.extend_from_slice(&expiration.to_be_bytes());
                bytes.extend_from_slice(key.as_bytes());
                bytes.append(&mut value);
            }
            Message::Request(Request::Delete(key)) => {
                bytes.push(MessageKind::DeleteRequest.into());
                bytes.extend_from_slice(&key.len().to_be_bytes());
                bytes.extend_from_slice(key.as_bytes());
            }
            Message::Response(Response::Ok) => {
                bytes.push(MessageKind::OkResponse.into());
            }
            Message::Response(Response::Error(msg)) => {
                bytes.push(MessageKind::ErrorResponse.into());
                bytes.extend_from_slice(&msg.len().to_be_bytes());
                bytes.append(&mut msg.into_bytes());
            }
            Message::Response(Response::Pong) => {
                bytes.push(MessageKind::PongResponse.into());
            }
            Message::Response(Response::Value(mut value)) => {
                bytes.push(MessageKind::ValueResponse.into());
                bytes.extend_from_slice(&value.len().to_be_bytes());
                bytes.append(&mut value);
            }
            Message::Response(Response::KeyNotFound) => {
                bytes.push(MessageKind::KeyNotFoundResponse.into());
            }
        }
        debug_assert!(bytes.len() <= sizes::MAX_HEADER_SIZE);
        bytes.resize(sizes::MAX_HEADER_SIZE, 0u8);
        self.stream.write_all(&bytes).await?;
        Ok(())
    }

    fn parse_version_header(&mut self, header: &[u8]) -> Result<Version, Error> {
        let version_bytes = &header[..sizes::VERSION_SIZE];
        let version = Version::from_be_bytes(
            version_bytes
                .try_into()
                .expect("version_bytes len != VERSION_SIZE"),
        );
        Ok(version)
    }

    fn parse_klen_header(&mut self, header: &[u8]) -> Result<KeyLen, Error> {
        let klen_bytes = &header[..sizes::KLEN_SIZE];
        let klen = KeyLen::from_be_bytes(
            klen_bytes
                .try_into()
                .expect("klen_bytes.len() should be equal to KLEN_SIZE"),
        );
        Ok(klen)
    }

    fn parse_klen_vlen_exp_header(
        &mut self,
        header: &[u8],
    ) -> Result<(KeyLen, ValueLen, Expiration), Error> {
        let (klen_bytes, rest) = header.split_at(sizes::KLEN_SIZE);
        let (vlen_bytes, rest) = rest.split_at(sizes::VLEN_SIZE);
        let expiration_bytes = &rest[..sizes::EXP_SIZE];

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

    fn parse_vlen_header(&mut self, header: &[u8]) -> Result<ValueLen, Error> {
        let vlen_bytes = &header[..sizes::VLEN_SIZE];
        assert_eq!(vlen_bytes.len(), sizes::VLEN_SIZE);
        let vlen = ValueLen::from_be_bytes(
            vlen_bytes
                .try_into()
                .expect("vlen_bytes.len() should be equal to VLEN_SIZE"),
        );

        Ok(vlen)
    }

    fn parse_utf8(buf: Vec<u8>) -> Result<String, Error> {
        Ok(String::from_utf8(buf).map_err(|_| ParseError::InvalidString)?)
    }
}

// test in submodule so we can access private stream
#[cfg(test)]
mod test {
    use super::*;

    struct MockStream {
        read_data: std::collections::VecDeque<u8>,
        wrote_data: Vec<u8>,
    }

    #[async_trait::async_trait]
    impl AsyncReader for MockStream {
        async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
            for byte in buf.iter_mut() {
                if self.read_data.is_empty() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        format!(
                            "read_exact: cannot completely fill buffer, stopped at: buf={:?}",
                            buf
                        ),
                    ));
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

    async fn assert_parsed(data: Vec<u8>, msg: Message) {
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
        let mut data: Vec<u8>;

        assert_eq!(21, sizes::MAX_HEADER_SIZE);

        // TODO: divide test cases to functions so it does not stop at first fail + reduce
        // boilerplate somehow
        // TODO!: tests for encoding

        data = vec![0];
        data.append(&mut vec![0, 1]);
        data.append(&mut vec![0; 18]);
        assert_parsed(data, Message::Request(Request::Version(1))).await;

        data = vec![1];
        data.append(&mut vec![0; 20]);
        assert_parsed(data, Message::Request(Request::Ping)).await;

        data = vec![2];
        data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 2]); // klen
        data.append(&mut vec![0; 12]); // rest of header
        data.append(&mut vec![97, 98]); // utf8 encoded key
        assert_parsed(data, Message::Request(Request::Get("ab".to_owned()))).await;

        data = vec![3];
        data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 2]); // klen
        data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 3]); // vlen
        data.append(&mut vec![0, 0, 1, 0]); // exp
        data.append(&mut vec![97, 98]); // utf8 encoded key
        data.append(&mut vec![1, 2, 3]); // value
        assert_parsed(
            data,
            Message::Request(Request::Set {
                key: "ab".to_owned(),
                value: vec![1, 2, 3],
                expiration: 256,
            }),
        )
        .await;

        data = vec![4];
        data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 2]); // klen
        data.append(&mut vec![0; 12]); // rest of header
        data.append(&mut vec![97, 98]); // utf8 encoded key
        assert_parsed(data, Message::Request(Request::Delete("ab".to_owned()))).await;

        data = vec![5];
        data.append(&mut vec![0; 20]);
        assert_parsed(data, Message::Request(Request::Clear)).await;

        data = vec![128];
        data.append(&mut vec![0; 20]);
        assert_parsed(data, Message::Response(Response::Pong)).await;

        data = vec![129];
        data.append(&mut vec![0; 20]);
        assert_parsed(data, Message::Response(Response::Ok)).await;

        data = vec![130];
        data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 4]); // vlen
        data.append(&mut vec![0; 12]); // rest of header
        data.append(&mut vec![1, 2, 3, 4]); // msg
        assert_parsed(data, Message::Response(Response::Value(vec![1, 2, 3, 4]))).await;

        data = vec![131];
        data.append(&mut vec![0; 20]);
        assert_parsed(data, Message::Response(Response::KeyNotFound)).await;

        data = vec![132];
        data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 3]); // vlen
        data.append(&mut vec![0; 12]); // rest of header
        data.append(&mut vec![101, 114, 114]); // msg
        assert_parsed(data, Message::Response(Response::Error("err".to_owned()))).await;
    }
}
