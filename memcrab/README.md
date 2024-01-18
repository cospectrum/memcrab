# memcrab

`memcrab` client.

## Examples

### RawClient

```rust
use memcrab::RawClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:80".parse()?;
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
