#[derive(Debug, Clone)]
pub enum Request {
    Get(String),
    Set(String, Vec<u8>),
    Delete(String),
    Clear,
    Ping,
}

#[derive(Debug, Clone)]
pub enum Response {
    Value(Vec<u8>),
    Executed,
    KeyNotFound,
    Pong,
}
