/*!
# memcrab

`memcrab` client.

## Usage

### RawClient

#### Tcp

```no_run
use memcrab::{RawClient, connections::Tcp, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "127.0.0.1:80".parse().unwrap();
    let mut client = RawClient::<Tcp>::connect(addr).await?;

    client.set("date", vec![2, 3, 24]).await?;
    let name = client.get("name").await?;
    match name {
        Some(val) => println!("got {:?} from cache", val),
        None => println!("cache miss for name"),
    }
    Ok(())
}
```
*/

#[allow(unused_variables)]
mod raw_client;

pub mod connections;

pub use memcrab_protocol::Error;
pub use raw_client::{RawClient, Rpc};

#[cfg(test)]
mod tests {}
