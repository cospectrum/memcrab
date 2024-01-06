use tokio::io::AsyncReadExt;

pub enum Error {
    IO(std::io::Error),
    Incomplete(Vec<u8>),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

pub struct StreamReader<S>(pub S);

impl<S> StreamReader<S>
where
    S: AsyncReadExt + Unpin,
{
    pub async fn read_exact(&mut self, size: usize) -> Result<Vec<u8>, Error> {
        let mut v = vec![0_u8; size];
        let mut got = 0;
        loop {
            if got == size {
                return Ok(v);
            }
            let slice = &mut v[got..];
            match self.0.read(slice).await? {
                0 => {
                    debug_assert!(got < size);
                    v.truncate(got);
                    return Err(Error::Incomplete(v));
                }
                n => {
                    got += n;
                }
            }
        }
    }
}
