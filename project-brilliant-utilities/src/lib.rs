#[derive(Debug)]
#[derive(Eq)]
pub struct NanoSecond(u64);
#[derive(Debug)]
#[derive(Eq)]
pub struct Currency(u64);
#[derive(Debug)]
#[derive(Eq)]
pub struct Token(u64);

impl PartialEq for NanoSecond {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub const NANOSECONDS_PER_DAY: Token = Token(86400000000000);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            NANOSECONDS_PER_DAY,
            Token(1_000_000_000 * 60 * 60 * 24)
        )
    }
}
