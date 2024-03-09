use async_trait::async_trait;
use tokio::net::TcpStream;

use crate::{Error, Rpc};
use memcrab_protocol::{Msg, ParseError, Request, Response, Socket};

pub struct Tcp {
    inner: Socket<TcpStream>,
}

impl Tcp {
    pub(crate) fn from_stream(stream: TcpStream) -> Self {
        let inner = Socket::new(stream);
        Self { inner }
    }
}

#[async_trait]
impl Rpc for Tcp {
    async fn call(&mut self, request: Request) -> Result<Response, Error> {
        self.inner.send(Msg::Request(request)).await?;
        match self.inner.recv().await? {
            Msg::Response(resp) => Ok(resp),
            _ => Err(Error::Parse(ParseError::UnknownMsgKind)),
        }
    }
}
