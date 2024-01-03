use core::num::NonZeroUsize;
use memcrab_cache::Cache;
use memcrab_server::start_grpc_server;
use std::time::Duration;
use tokio::task;

#[allow(clippy::single_match)]
#[tokio::test]
async fn test_run() {
    let addr = "[::1]:50051".parse().unwrap();

    let maxbytes = 100_000;
    let maxlen = NonZeroUsize::new(110).unwrap();
    let cache = Cache::new(maxlen, maxbytes);

    let join = task::spawn(async move {
        start_grpc_server(addr, cache).await.unwrap();
    });

    let done = tokio::time::timeout(Duration::from_secs(15), join).await;
    match done {
        Ok(_) => panic!("grpc server stopped"),
        Err(_) => {}
    }
}
