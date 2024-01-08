use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
pub enum ResponseKind {
    Ok = 0,
    Err = 1,
    Pong = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ok = 0;
        match ok.try_into().unwrap() {
            ResponseKind::Ok => (),
            variant => panic!("got unexpected variant: {:?}", variant),
        }
    }
}
