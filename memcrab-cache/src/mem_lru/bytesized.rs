pub trait ByteSized {
    fn bytesize(&self) -> usize;
}

impl ByteSized for Vec<u8> {
    fn bytesize(&self) -> usize {
        self.capacity()
    }
}

impl ByteSized for String {
    fn bytesize(&self) -> usize {
        self.capacity()
    }
}
