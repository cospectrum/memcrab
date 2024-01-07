use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::ProtocolError;

#[async_trait::async_trait]
pub trait AsyncReader<Err = ProtocolError> {
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<usize, Err>;
}

#[async_trait::async_trait]
impl AsyncReader for OwnedReadHalf {
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<usize, ProtocolError> {
        let n = AsyncReadExt::read_exact(self, buf).await?;
        Ok(n)
    }
}

#[async_trait::async_trait]
pub trait AsyncWriter<Err = ProtocolError> {
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Err>;
}

#[async_trait::async_trait]
impl AsyncWriter for OwnedWriteHalf {
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), ProtocolError> {
        Ok(AsyncWriteExt::write_all(self, buf).await?)
    }
}
