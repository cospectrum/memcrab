/*!
# memcrab-server

## Usage

```no_run
use memcrab_server::{serve, Cache};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let gb = 2_usize.pow(30);
    let cache = Cache::builder()
        .segments(10)
        .max_bytesize(gb)
        .build()
        .into();

    let listener = TcpListener::bind("127.0.0.1:9900").await.unwrap();
    serve(listener, cache).await.unwrap();
}
```
*/

mod cache;
mod serve;

pub use cache::Cache;
pub use serve::{serve, AcceptConnection};

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use memcrab_protocol::{Msg, Request, Response, Socket};
    use tokio::net::{TcpListener, TcpStream};

    use super::*;

    #[tokio::test]
    async fn test_init() {
        let gb = 2_usize.pow(30);
        let cache = Cache::builder()
            .segments(10)
            .max_bytesize(gb)
            .build()
            .into();

        let addr = "127.0.0.1:9900";
        let listener = TcpListener::bind(addr).await.unwrap();

        tokio::spawn(async move {
            serve(listener, cache).await.unwrap();
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

        let value = vec![3, 4];
        socket
            .send(Msg::Request(Request::Set {
                key: key.clone(),
                value: value.clone(),
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
        assert_eq!(msg, Msg::Response(Response::Value(value.clone())));
    }
}
