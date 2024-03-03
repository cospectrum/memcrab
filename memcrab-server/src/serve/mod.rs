mod err;
mod listener;
mod server;
mod socket;

use std::io;

use crate::Cache;
use err::ServerSideError;
use memcrab_protocol::{AsyncRead, AsyncWrite};

pub use listener::AcceptConnection;

pub async fn serve<S>(listener: impl AcceptConnection<Stream = S>, cache: Cache) -> io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    server::start_server(listener, cache).await
}
