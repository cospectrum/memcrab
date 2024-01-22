use super::ServerSideError;
use memcrab_protocol::{AsyncReader, AsyncWriter, Msg, Request, Response, Socket};

#[derive(Debug, Clone)]
pub struct ServerSocket<S> {
    inner: Socket<S>,
}

impl<S> From<Socket<S>> for ServerSocket<S> {
    fn from(inner: Socket<S>) -> Self {
        Self { inner }
    }
}

impl<S> ServerSocket<S>
where
    S: AsyncReader + AsyncWriter + Send,
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
