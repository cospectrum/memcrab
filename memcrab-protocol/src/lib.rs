use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

type Bytes = Vec<u8>;

fn four_bytes_to_usize(bytes: &[u8]) -> usize {
    assert_eq!(bytes.len(), 4);

    (bytes[0] as usize) << 24
        | (bytes[1] as usize) << 16
        | (bytes[2] as usize) << 8
        | (bytes[3] as usize)
}

#[async_trait::async_trait]
pub trait AsyncReader {
    async fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

#[async_trait::async_trait]
impl AsyncReader for OwnedReadHalf {
    async fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        AsyncReadExt::read_exact(self, buf).await
    }
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    Get(Bytes),
    Set(Bytes, Bytes),
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Incomplete,
}

impl From<std::io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

pub struct Producer<R: AsyncReader> {
    reader: R,
}

impl<R: AsyncReader> Producer<R> {
    pub fn new(reader: R) -> Self {
        Producer {
            reader,
        }
    }
    async fn next_chunk(&mut self, size: usize) -> Result<Bytes, Error> {
        let mut buf = vec![0u8; size];
        self.reader.read_exact(&mut buf).await?;
        Ok(buf)
    }
    pub async fn next_msg(&mut self) -> Result<Msg, Error> {
        let msg_type = self.next_chunk(1).await?;
        let key_len = self.next_chunk(1).await?;
        let key = self.next_chunk(key_len[0].into()).await?;
        if msg_type[0] == 0 {
            return Ok(Msg::Get(key));
        }
        let data_len = self.next_chunk(4).await?;
        let data = self
            .next_chunk(four_bytes_to_usize(&data_len))
            .await?;
        Ok(Msg::Set(key, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockReader {
        parts: Vec<Vec<u8>>,
        remainder: Vec<u8>,
    }

    impl MockReader {
        fn new(parts: Vec<Vec<u8>>) -> Self {
            MockReader {
                parts,
                remainder: vec![],
            }
        }
    }

    #[async_trait::async_trait]
    impl AsyncReader for MockReader {
        async fn read_exact(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            println!("{:?} {:?}", self.parts, self.remainder);
            todo!();
        }
    }

    #[tokio::test]
    async fn test_get() {
        let mock_reader = MockReader::new( 
            vec![vec![0, 2, 1, 1]],
        );
        let mut producer = Producer::new(mock_reader);

        let msg = producer.next_msg().await;
        assert_eq!(msg.unwrap(), Msg::Get(vec![1, 1]))
    }

    #[tokio::test]
    async fn test_get_with_partitions() {
        let mock_reader = MockReader::new(vec![vec![0], vec![3, 1], vec![2, 3]]);
        let mut producer = Producer::new(mock_reader);

        let msg = producer.next_msg().await;
        assert_eq!(msg.unwrap(), Msg::Get(vec![1, 2, 3]))
    }

    #[tokio::test]
    async fn test_set() {
        let mock_reader = MockReader::new(
            vec![vec![1, 1, 1, 0, 0, 0, 3, 8, 8, 8]],
        );
        let mut producer = Producer::new(mock_reader);

        let msg = producer.next_msg().await;
        assert_eq!(msg.unwrap(), Msg::Set(vec![1], vec![8, 8, 8,]))
    }
}
