use super::ServerSideError;
use memcrab_protocol::{AsyncRead, AsyncWrite, Msg, Request, Response, Socket};

#[derive(Debug, Clone)]
pub struct ServerSocket<S> {
    inner: Socket<S>,
}

impl<S> ServerSocket<S> {
    pub fn new(inner: Socket<S>) -> Self {
        Self { inner }
    }
    pub fn from_stream(stream: S) -> Self {
        Socket::new(stream).into()
    }
}

impl<S> From<Socket<S>> for ServerSocket<S> {
    fn from(inner: Socket<S>) -> Self {
        Self::new(inner)
    }
}

impl<S> ServerSocket<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn recv(&mut self) -> Result<Request, ServerSideError> {
        let msg = self.inner.recv().await?;
        match msg {
            Msg::Request(req) => Ok(req),
            Msg::Response(_) => Err(ServerSideError::InvalidMsg),
        }
    }
    pub async fn send(&mut self, response: Response) -> Result<(), ServerSideError> {
        self.inner.send(Msg::Response(response)).await?;
        Ok(())
    }
}
