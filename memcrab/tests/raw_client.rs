use memcrab::RawClient;

async fn _test_raw_client() -> anyhow::Result<()> {
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
