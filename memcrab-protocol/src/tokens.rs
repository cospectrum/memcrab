#[derive(Debug, Clone)]
pub enum Msg {
    Get(Vec<u8>),
    Set(Vec<u8>, Vec<u8>),
}
