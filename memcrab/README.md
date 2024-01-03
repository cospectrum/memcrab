# memcrab

`memcrab` client.

## Examples

### RawClient

```rust
use memcrab::RawClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "http://[::1]:50051";
    let client = RawClient::connect(addr).await?;

    client.set("age", vec![0, 21]).await?;
    client.set("year", "2024".into()).await?;

    let name = client.get("name").await?;
    match name {
        Some(val) => println!("got {:?} from cache", val),
        None => println!("cache miss for name"),
    }
    Ok(())
}
```

### tonic

```rust
use memcrab::pb::{cache_rpc_client::CacheRpcClient, GetRequest, SetRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "http://[::1]:50051";
    let mut client = CacheRpcClient::connect(addr).await?;

    let msg = GetRequest {
        key: "name".to_owned(),
    };
    let req = tonic::Request::new(msg);
    let resp = client.get(req).await?.into_inner();
    match resp.value {
        Some(val) => {
            println!("got bytes from cache: {:?}", val);
        }
        None => {
            println!("no value in cache");
        }
    }

    let msg = SetRequest {
        key: "fullname".to_owned(),
        value: vec![1, 3, 4, 5, 6, 7],
    };
    let req = tonic::Request::new(msg);
    client.set(req).await?;
    Ok(())
}
```
