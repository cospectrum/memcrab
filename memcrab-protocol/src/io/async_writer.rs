use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::MemcrabError;

#[async_trait::async_trait]
pub trait AsyncWriter<Err = MemcrabError> {
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Err>;
}

#[async_trait::async_trait]
impl AsyncWriter for TcpStream {
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), MemcrabError> {
        AsyncWriteExt::write_all(self, buf).await?;
        Ok(())
    }
}
