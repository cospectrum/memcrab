use memcrab::{connections::Tcp, Error, RawClient};

#[allow(dead_code)]
async fn tcp_raw_client_readme() -> Result<(), Error> {
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
