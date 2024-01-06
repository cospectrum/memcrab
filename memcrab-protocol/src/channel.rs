use crate::tokens::Msg;
use crate::{AsyncReader, ProtocolError};

#[allow(dead_code)]
pub struct MemcrabChannel<R: AsyncReader> {
    reader: R,
}

impl<R> MemcrabChannel<R>
where
    R: AsyncReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    #[allow(dead_code)]
    async fn next_chunk(&mut self, size: usize) -> Result<Vec<u8>, ProtocolError> {
        let mut buf = vec![0; size];
        let n = self.reader.read_exact(&mut buf).await?;
        if n != size {
            return Err(ProtocolError::IncompleteRead {
                expected_size: size,
                buf,
            });
        }
        Ok(buf)
    }
    pub async fn next_msg(&mut self) -> Result<Msg, ProtocolError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLinearReader {
        buf: Vec<u8>,
        start: usize,
    }

    impl MockLinearReader {
        fn new(buf: Vec<u8>) -> Self {
            Self { buf, start: 0 }
        }
    }

    #[async_trait::async_trait]
    impl AsyncReader for MockLinearReader {
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
        let mock_reader = MockLinearReader::new(vec![0, 2, 1, 1]);
        let mut producer = MemcrabChannel::new(mock_reader);

        let chunk = producer.next_chunk(2).await.unwrap();
        assert_eq!(chunk, [0, 2])
    }
}
