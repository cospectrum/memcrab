use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

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
