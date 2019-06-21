#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NanoSecond(u64);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Currency(u64);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token(u64);

pub const NANOSECONDS_PER_DAY: Token = Token(86_400_000_000_000);

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
