#[allow(clippy::all)]
#[rustfmt::skip]
mod pb;

mod service;

use pb::cache_rpc_server::CacheRpcServer;
use service::Service;
use std::net::SocketAddr;
use tonic::transport::Server;

pub async fn start_grpc_server(
    addr: SocketAddr,
    cache: memcrab_cache::Cache,
) -> Result<(), tonic::transport::Error> {
    let cache_srvice = Service::new(cache);
    Server::builder()
        .add_service(CacheRpcServer::new(cache_srvice))
        .serve(addr)
        .await
}
