use core::panic;
use memcrab_protocol::{AsyncRead, AsyncWrite, Error as ProtocolError, Request, Response};
use std::{io, num::NonZeroU32, sync::Arc};
use tracing::info;

use super::{listener::AcceptConnection, socket::ServerSocket};
use crate::{cache::Cache, serve::err::ServerSideError};

pub(super) async fn start_server<S>(
    listener: impl AcceptConnection<Stream = S>,
    cache: Cache,
) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    info!("started memcrab server...");
    let cache = Arc::new(cache);
    loop {
        let stream = listener.accept_connection().await?;
        let socket = ServerSocket::from_stream(stream);
        let cache = cache.clone();
        tokio::spawn(async move { handle(socket, cache).await });
    }
}

async fn handle<S>(mut socket: ServerSocket<S>, cache: Arc<Cache>)
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let cache = cache.as_ref();
    loop {
        let request = match socket.recv().await {
            Ok(req) => req,
            Err(ServerSideError::Protocol(ProtocolError::IO(err))) => match err.kind() {
                io::ErrorKind::UnexpectedEof => {
                    info!("eof, close connection");
                    return;
                }
                _ => panic!("{:?}", err),
            },
            err => panic!("{:?}", err),
        };
        info!("received request: {:?}", &request);
        let response = response_to(request, cache);
        info!("sending response: {:?}", &response);
        socket.send(response).await.unwrap();
    }
}

fn response_to(request: Request, cache: &Cache) -> Response {
    match request {
        Request::Ping => Response::Pong,
        Request::Get(ref key) => match cache.get(key) {
            Some(val) => Response::Value(val),
            None => Response::KeyNotFound,
        },
        Request::Delete(ref key) => match cache.remove(key) {
            Some(_) => Response::Ok,
            None => Response::KeyNotFound,
        },
        Request::Set {
            key,
            value,
            expiration,
        } => {
            if expiration == 0 {
                cache.set(key, value)
            } else {
                let exp = NonZeroU32::new(expiration).unwrap();
                cache.set_with_expiration(key, value, exp);
            }
            Response::Ok
        }
        Request::Clear => {
            cache.clear();
            Response::Ok
        }
    }
}
