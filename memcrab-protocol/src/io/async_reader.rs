use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[async_trait::async_trait]
pub trait AsyncReader<E = std::io::Error> {
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), E>;

    async fn read_chunk(&mut self, size: usize) -> Result<Vec<u8>, E> {
        let mut buf = vec![0; size];
        self.read_exact(&mut buf).await?;
        Ok(buf)
    }
}

#[async_trait::async_trait]
impl AsyncReader for TcpStream {
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        let n = AsyncReadExt::read_exact(self, buf).await?;
        assert_eq!(n, buf.len());
        Ok(())
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
        async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
            let size = buf.len();
            let v = &self.buf[self.start..self.start + size];
            assert_eq!(v.len(), buf.len());
            for (&src, dst) in v.iter().zip(buf) {
                *dst = src;
            }
            self.start += size;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_chunk() {
        let mut mock_stream = MockLinearStream::new(vec![0, 2, 1, 1]);

        let chunk = mock_stream.read_chunk(2).await.unwrap();
        assert_eq!(chunk, [0, 2]);
        let chunk = mock_stream.read_chunk(2).await.unwrap();
        assert_eq!(chunk, [1, 1]);
    }
}
