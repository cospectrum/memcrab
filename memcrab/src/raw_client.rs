use crate::{connections::*, Error};
use memcrab_protocol::{Msg, Request, Response};
use std::{net::SocketAddr, path::Path};

#[async_trait::async_trait]
pub trait Rpc
where
    Self: Sized,
{
    async fn call(&mut self, request: Request) -> Result<Response, Error>;
}

pub struct RawClient<C> {
    conn: C,
}

impl<C> RawClient<C> {
    fn new(conn: C) -> Self {
        Self { conn }
    }
}

fn invalid_resp(resp: Response) -> Error {
    Error::InvalidMsg(Msg::Response(resp))
}

impl<C> RawClient<C>
where
    C: Rpc,
{
    pub async fn get(&mut self, key: impl Into<String>) -> Result<Option<Vec<u8>>, Error> {
        let key = key.into();
        match self.conn.call(Request::Get(key)).await? {
            Response::Value(val) => Ok(Some(val)),
            Response::KeyNotFound => Ok(None),
            resp => Err(invalid_resp(resp)),
        }
    }
    pub async fn set(&mut self, key: impl Into<String>, value: Vec<u8>) -> Result<(), Error> {
        let key = key.into();
        let request = Request::Set {
            key,
            value,
            expiration: 0,
        };
        match self.conn.call(request).await? {
            Response::Ok => Ok(()),
            resp => Err(invalid_resp(resp)),
        }
    }
}

impl RawClient<Tcp> {
    pub async fn connect(addr: SocketAddr) -> Result<Self, Error> {
        use tokio::net::TcpStream;

        let stream = TcpStream::connect(addr).await?;
        Ok(Self::new(Tcp::from_stream(stream)))
    }
}

#[cfg(target_family = "unix")]
impl RawClient<Unix> {
    pub async fn connect(path: impl AsRef<Path>) -> Result<Self, Error> {
        use tokio::net::UnixStream;

        let stream = UnixStream::connect(path).await?;
        Ok(Self::new(Unix::from_stream(stream)))
    }
}
