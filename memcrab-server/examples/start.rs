use memcrab_server::{serve, Cache};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let one_gb = 2usize.pow(30);
    let cache = Cache::builder()
        .segments(10)
        .max_bytesize(one_gb)
        .build()
        .into();

    let listener = TcpListener::bind("127.0.0.1:9900").await.unwrap();
    serve(listener, cache).await.unwrap();
}
