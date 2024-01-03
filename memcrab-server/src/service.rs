use std::sync::Mutex;

use memcrab_cache::Cache;
use tonic::{Code, Request, Response, Status};

use crate::pb::{cache_rpc_server::CacheRpc, GetRequest, GetResponse, SetRequest, SetResponse};

#[derive(Debug)]
pub struct Service {
    cache: Mutex<Cache>,
    maxbytes: usize,
}

impl Service {
    pub fn new(cache: Cache) -> Self {
        let maxbytes = cache.maxbytes();
        Self {
            cache: Mutex::new(cache),
            maxbytes,
        }
    }
}

#[tonic::async_trait]
impl CacheRpc for Service {
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = req.into_inner();
        let key = req.key;

        let mut cache = self.cache.lock().unwrap();
        let option = cache.get(&key).cloned();

        let reply = GetResponse { value: option };
        Ok(Response::new(reply))
    }

    async fn set(&self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let req = req.into_inner();
        let key = req.key;
        let val = req.value;
        let sizeof_item = Cache::size_of(&key, &val);
        if sizeof_item > self.maxbytes {
            let msg = format!(
                "Item is too large: {} bytes, expected size <= {} bytes.",
                sizeof_item, self.maxbytes
            );
            return Err(Status::new(Code::InvalidArgument, msg));
        }
        let mut cache = self.cache.lock().unwrap();
        cache.set(key, val);

        let reply = SetResponse {};
        Ok(Response::new(reply))
    }
}
