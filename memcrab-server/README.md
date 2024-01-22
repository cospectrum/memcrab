# memcrab-server

```rs
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
