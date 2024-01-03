# memcrab-server

```rust
use core::num::NonZeroUsize;
use memcrab_server::start_grpc_server;
use memcrab_cache::Cache;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    let maxbytes = 100_000;
    let maxlen = NonZeroUsize::new(110).unwrap();
    let cache = Cache::new(maxlen, maxbytes);

    start_grpc_server(addr, cache).await.unwrap();
}
```
