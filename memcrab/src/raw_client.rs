use tonic::{transport::Channel, Status};

use crate::pb::{cache_rpc_client::CacheRpcClient, GetRequest, SetRequest};

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct RawClient {
    inner: CacheRpcClient<Channel>,
}

impl RawClient {
    pub async fn connect<D>(endpoint: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let inner = CacheRpcClient::connect(endpoint).await?;
        Ok(Self { inner })
    }
    pub async fn get(&self, key: impl Into<String>) -> Result<Option<Vec<u8>>, Status> {
        let key = key.into();
        let msg = GetRequest { key };
        let req = tonic::Request::new(msg);
        let resp = self.inner.clone().get(req).await?;
        Ok(resp.into_inner().value)
    }
    pub async fn set(&self, key: impl Into<String>, value: Vec<u8>) -> Result<(), Status> {
        let key = key.into();
        let msg = SetRequest { key, value };
        let req = tonic::Request::new(msg);
        self.inner.clone().set(req).await?;
        Ok(())
    }
}
