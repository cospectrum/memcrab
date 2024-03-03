# memcrab-server

## Usage

### Tcp

```rs
use memcrab_server::{serve, Cache};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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

## Examples

Start TCP server on "127.0.0.1:9900"
```sh
cargo run --example start
```
