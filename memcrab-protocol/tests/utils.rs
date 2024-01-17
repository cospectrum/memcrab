use memcrab_protocol::io::{AsyncReader, AsyncWriter};
use std::{
    collections::VecDeque,
    io::ErrorKind,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct MockStream {
    pub data: Arc<Mutex<VecDeque<u8>>>,
}

impl MockStream {
    pub fn new() -> Self {
        let deq = VecDeque::new();
        let data = Arc::new(Mutex::new(deq));
        Self { data }
    }
    #[allow(unused)]
    pub fn len(&self) -> usize {
        let deq = self.data.lock().unwrap();
        return deq.len();
    }
    pub fn is_empty(&self) -> bool {
        let deq = self.data.lock().unwrap();
        return deq.is_empty();
    }
}

#[async_trait::async_trait]
impl AsyncReader for MockStream {
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        use std::io::Error;

        let mut deq = self.data.lock().unwrap();
        if deq.len() < buf.len() {
            let msg = format!(
                "read_exact: expected buf.len() == {:?}, stream.len() == {:?}",
                buf.len(),
                deq.len(),
            );
            return Err(Error::new(ErrorKind::UnexpectedEof, msg));
        }

        for dst in buf.iter_mut() {
            let src = deq.pop_front().unwrap();
            *dst = src;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncWriter for MockStream {
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        let mut deq = self.data.lock().unwrap();
        for &byte in buf {
            deq.push_back(byte)
        }
        Ok(())
    }
}
