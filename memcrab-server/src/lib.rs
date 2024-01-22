/*!
# memcrab-server

## Usage

```no_run
use memcrab_server::{Server, CacheCfg};

#[tokio::main]
async fn main() {
    let gb = 2_usize.pow(30);
    let cfg = CacheCfg::builder()
        .segments(10)
        .max_bytesize(gb)
        .build();

    let addr = "127.0.0.1:9900".parse().unwrap();
    let server = Server::from(cfg);
    server.start(addr).await.unwrap();
}
```
*/

mod cache;
mod serve;

use cache::Cache;

pub use cache::CacheCfg;
pub use serve::Server;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use memcrab_protocol::{Msg, Request, Response, Socket};
    use tokio::net::TcpStream;

    use super::*;

    #[tokio::test]
    async fn test_init() {
        let gb = 2_usize.pow(30);
        let cfg = CacheCfg::builder().segments(10).max_bytesize(gb).build();

        let addr = "127.0.0.1:9900".parse().unwrap();

        let server = Server::from(cfg);
        tokio::spawn(async move {
            server.start(addr).await.unwrap();
        });
        tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

        let stream = tokio::time::timeout(Duration::from_secs_f32(0.1), TcpStream::connect(addr))
            .await
            .expect("connect")
            .unwrap();
        let mut socket = Socket::new(stream);

        socket.send(Msg::Request(Request::Ping)).await.unwrap();
        let msg = socket.recv().await.unwrap();
        assert_eq!(msg, Msg::Response(Response::Pong));

        let key = "some-key".to_string();
        socket
            .send(Msg::Request(Request::Get(key.clone())))
            .await
            .unwrap();
        let msg = socket.recv().await.unwrap();
        assert_eq!(msg, Msg::Response(Response::KeyNotFound));

        socket
            .send(Msg::Request(Request::Set {
                key: key.clone(),
                value: vec![3, 4],
                expiration: 0,
            }))
            .await
            .unwrap();
        let msg = socket.recv().await.unwrap();
        assert_eq!(msg, Msg::Response(Response::Ok));

        socket
            .send(Msg::Request(Request::Get(key.clone())))
            .await
            .unwrap();
        let msg = socket.recv().await.unwrap();
        assert_eq!(msg, Msg::Response(Response::Value(vec![3, 4])));
    }
}
