use std::io;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tracing::info;

#[async_trait::async_trait]
pub trait AcceptConnection {
    type Stream;

    async fn accept_connection(&self) -> io::Result<Self::Stream>;
}

#[async_trait::async_trait]
impl AcceptConnection for TcpListener {
    type Stream = TcpStream;

    async fn accept_connection(&self) -> io::Result<Self::Stream> {
        let (socket, addr) = self.accept().await?;
        info!("accepted tcp connection, addr: {:?}", addr);
        Ok(socket)
    }
}

#[async_trait::async_trait]
impl AcceptConnection for UnixListener {
    type Stream = UnixStream;

    async fn accept_connection(&self) -> io::Result<Self::Stream> {
        let (socket, addr) = self.accept().await?;
        info!("accepted unix connection, addr: {:?}", addr);
        Ok(socket)
    }
}
