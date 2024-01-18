// use std::net::SocketAddr;
//
// use memcrab_protocol::ClientSideError;
//
// pub struct RawClient {}
//
// impl RawClient {
//     pub async fn connect(addr: SocketAddr) -> Result<Self, ClientSideError> {
//         todo!()
//     }
//     pub async fn get(&self, key: impl Into<String>) -> Result<Option<Vec<u8>>, ClientSideError> {
//         let key = key.into();
//         todo!()
//     }
//     pub async fn set(&self, key: impl Into<String>, value: Vec<u8>) -> Result<(), ClientSideError> {
//         let key = key.into();
//         todo!()
//     }
// }
