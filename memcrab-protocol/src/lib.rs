use std::io;
use tokio::net::tcp::OwnedReadHalf;
use tokio::io::AsyncReadExt;

type Bytes = Vec<u8>;

fn four_bytes_to_usize(bytes: &[u8]) -> usize {
    assert_eq!(bytes.len(), 4);

    (bytes[0] as usize) << 24
        | (bytes[1] as usize) << 16
        | (bytes[2] as usize) << 8
        | (bytes[3] as usize)
}

use tokio::io::Result;

#[async_trait::async_trait]
pub trait AsyncReader {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

#[async_trait::async_trait]
impl AsyncReader for OwnedReadHalf {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        AsyncReadExt::read(self, buf).await
    }
}

pub struct Consumer {
    buffer: Bytes,
    len: usize,
}

impl Consumer {
    fn new() -> Consumer {
        Consumer {
            buffer: Vec::new(),
            len: 0,
        }
    }

    fn set_len(&mut self, length: usize) {
        self.len = length;
    }

    fn build(&mut self) -> Option<Bytes> {
        if self.buffer.len() >= self.len {
            Some(self.buffer.drain(0..self.len).collect())
        } else {
            None
        }
    }

    fn consume(&mut self, bytes: &mut [u8]) {
        self.buffer.append(&mut bytes.to_vec());
    }
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    Get(Bytes),
    Set(Bytes, Bytes),
}

pub struct Producer<R: AsyncReader> {
    reader: R,
    consumer: Consumer,
}

impl<R: AsyncReader> Producer<R> {
    pub fn new(reader: R) -> Self {
        Producer {
            reader,
            consumer: Consumer::new(),
        }
    }
    async fn next_chunk(&mut self, size: usize) -> io::Result<Option<Bytes>> {
        self.consumer.set_len(size);
        loop {
            let mut buf = [0; 128];

            match self.reader.read(&mut buf).await {
                Ok(0) => return Ok(self.consumer.build()),
                Ok(n) => {
                    self.consumer.consume(&mut buf[0..n]);

                    if let Some(bytes) = self.consumer.build() {
                        println!("new chunk produced: {:?}", bytes);
                        return Ok(Some(bytes));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
    pub async fn next_msg(&mut self) -> io::Result<Option<Msg>> {
        let msg_type = self.next_chunk(1).await?.unwrap();
        let key_len = self.next_chunk(1).await?.unwrap();
        let key = self.next_chunk(key_len[0].into()).await?.unwrap();
        if msg_type[0] == 0 {
            return Ok(Some(Msg::Get(key)));
        }
        let data_len = self.next_chunk(4).await?.unwrap();
        let data = self
            .next_chunk(four_bytes_to_usize(&data_len))
            .await?
            .unwrap();
        Ok(Some(Msg::Set(key, data)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockReader {
        parts: Vec<Vec<u8>>,
    }

    #[async_trait::async_trait]
    impl AsyncReader for MockReader {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            if !self.parts.is_empty() {
                let part = self.parts.remove(0);
                for (i, b) in part.iter().enumerate() {
                    buf[i] = *b;
                }
                Ok(part.len())
            } else {
                Ok(0)
            }
        }
    }

    #[tokio::test]
    async fn test_get() {
        let mock_reader = MockReader {
            parts: vec![vec![0, 2, 1, 1].into()],
        };
        let mut producer = Producer::new(mock_reader);

        let msg = producer.next_msg().await;
        assert_eq!(msg.unwrap().unwrap(), Msg::Get(vec![1, 1]))
    }

    #[tokio::test]
    async fn test_get_with_partitions() {
        let mock_reader = MockReader {
            parts: vec![vec![0].into(), vec![3, 1].into(), vec![2, 3].into()],
        };
        let mut producer = Producer::new(mock_reader);

        let msg = producer.next_msg().await;
        assert_eq!(msg.unwrap().unwrap(), Msg::Get(vec![1, 2, 3]))
    }

    #[tokio::test]
    async fn test_set() {
        let mock_reader = MockReader {
            parts: vec![vec![1, 1, 1, 0, 0, 0, 3, 8, 8, 8]].into(),
        };
        let mut producer = Producer::new(mock_reader);

        let msg = producer.next_msg().await;
        assert_eq!(msg.unwrap().unwrap(), Msg::Set(vec![1], vec![8, 8, 8,]))
    }
}
