use memcrab_protocol::{Request, Response, Socket};
use std::{net::SocketAddr, num::NonZeroU32, sync::Arc};
use tokio::net::{TcpListener, TcpStream};

use crate::{Cache, CacheCfg};

use super::socket::ServerSocket;

pub struct Server {
    cache: Cache,
}

impl From<CacheCfg> for Server {
    fn from(cfg: CacheCfg) -> Self {
        Self::new(Cache::from(cfg))
    }
}

async fn start_server(server: Server, addr: SocketAddr) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr).await?;
    let server = Arc::new(server);
    loop {
        let server = server.clone();
        let (socket, client_addr) = listener.accept().await?;
        tokio::spawn(async move { server.handle(socket, client_addr).await });
    }
}

// TODO: logging
impl Server {
    fn new(cache: Cache) -> Self {
        Self { cache }
    }
    pub async fn start(self, addr: SocketAddr) -> Result<(), std::io::Error> {
        start_server(self, addr).await
    }
    // TODO: error handling
    async fn handle(&self, stream: TcpStream, _: SocketAddr) {
        let mut socket: ServerSocket<_> = Socket::new(stream).into();
        loop {
            let request = socket.recv().await.unwrap();
            let response = self.response_to(request);
            socket.send(response).await.unwrap();
        }
    }
    fn response_to(&self, request: Request) -> Response {
        match request {
            Request::Ping => Response::Pong,
            Request::Get(ref key) => match self.cache.get(key) {
                Some(val) => Response::Value(val),
                None => Response::KeyNotFound,
            },
            Request::Delete(ref key) => match self.cache.remove(key) {
                Some(_) => Response::Ok,
                None => Response::KeyNotFound,
            },
            Request::Set {
                key,
                value,
                expiration,
            } => {
                if expiration == 0 {
                    self.cache.set(key, value)
                } else {
                    let exp = NonZeroU32::new(expiration).unwrap();
                    self.cache.set_with_expiration(key, value, exp);
                }
                Response::Ok
            }
            Request::Clear => {
                self.cache.clear();
                Response::Ok
            }
        }
    }
}
