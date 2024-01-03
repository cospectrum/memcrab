use memcrab::pb::{cache_rpc_client::CacheRpcClient, GetRequest, SetRequest};

async fn _test_client() {
    let dst_addr = "http://[::1]:50051";
    let mut client = CacheRpcClient::connect(dst_addr).await.unwrap();

    let msg = GetRequest {
        key: "name".to_owned(),
    };
    let req = tonic::Request::new(msg);
    let resp = client.get(req).await.unwrap().into_inner();

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
    client.set(req).await.unwrap();
}
