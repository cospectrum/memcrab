use std::io;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

#[async_trait::async_trait]
pub trait AcceptConnection {
    type Stream;

    async fn accept_connection(&self) -> io::Result<Self::Stream>;
}

#[async_trait::async_trait]
impl AcceptConnection for TcpListener {
    type Stream = TcpStream;

    async fn accept_connection(&self) -> io::Result<Self::Stream> {
        let (socket, _) = self.accept().await?;
        Ok(socket)
    }
}

#[async_trait::async_trait]
impl AcceptConnection for UnixListener {
    type Stream = UnixStream;

    async fn accept_connection(&self) -> io::Result<Self::Stream> {
        let (socket, _) = self.accept().await?;
        Ok(socket)
    }
}
