use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[async_trait::async_trait]
pub trait AsyncWriter<E = std::io::Error> {
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), E>;
}

#[async_trait::async_trait]
impl AsyncWriter for TcpStream {
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        AsyncWriteExt::write_all(self, buf).await?;
        Ok(())
    }
}
