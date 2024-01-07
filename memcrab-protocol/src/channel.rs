use crate::{AsyncReader, ProtocolError};

#[allow(dead_code)]
pub struct MemcrabChannel<S: AsyncReader> {
    stream: S,
}

impl<S> MemcrabChannel<S>
where
    S: AsyncReader,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
    #[allow(dead_code)]
    async fn next_chunk(&mut self, size: usize) -> Result<Vec<u8>, ProtocolError> {
        let mut buf = vec![0; size];
        let n = self.stream.read_exact(&mut buf).await?;
        if n != size {
            return Err(ProtocolError::IncompleteRead {
                expected_size: size,
                buf,
            });
        }
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLinearStream {
        buf: Vec<u8>,
        start: usize,
    }

    impl MockLinearStream {
        fn new(buf: Vec<u8>) -> Self {
            Self { buf, start: 0 }
        }
    }

    #[async_trait::async_trait]
    impl AsyncReader for MockLinearStream {
        async fn read_exact(&mut self, buf: &mut [u8]) -> Result<usize, ProtocolError> {
            let size = buf.len();
            let v = &self.buf[self.start..size];
            if v.len() != size {
                return Err(ProtocolError::IncompleteRead {
                    expected_size: size,
                    buf: buf.into(),
                });
            }

            assert_eq!(v.len(), buf.len());
            for (&src, dst) in v.iter().zip(buf) {
                *dst = src;
            }
            self.start += size;
            Ok(size)
        }
    }

    #[tokio::test]
    async fn test_chunk() {
        let mock_stream = MockLinearStream::new(vec![0, 2, 1, 1]);
        let mut channel = MemcrabChannel::new(mock_stream);

        let chunk = channel.next_chunk(2).await.unwrap();
        assert_eq!(chunk, [0, 2])
    }
}
