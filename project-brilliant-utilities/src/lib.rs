#[macro_use]
extern crate derive_more;

use std::ops::{Mul, Div, Rem};

pub const NANOSECONDS_PER_DAY: Token = Token(86_400_000_000_000);

#[derive(Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign)]
pub struct NanoSecond(u64);
#[derive(Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign)]
pub struct Currency(u64);
#[derive(Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign)]
pub struct Token(u64);

impl Mul<Token> for NanoSecond {
    type Output = Currency;
    fn mul(self: NanoSecond, rhs: Token) -> Currency {
        Currency(self.0 * rhs.0)
    }
}
impl Mul<&Token> for NanoSecond {
    type Output = Currency;
    fn mul(self: NanoSecond, rhs: &Token) -> Currency {
        Currency(self.0 * rhs.0)
    }
}
impl Mul<NanoSecond> for Token {
    type Output = Currency;
    fn mul(self: Token, rhs: NanoSecond) -> Currency {
        Currency(self.0 * rhs.0)
    }
}
impl Mul<&NanoSecond> for Token {
    type Output = Currency;
    fn mul(self: Token, rhs: &NanoSecond) -> Currency {
        Currency(self.0 * rhs.0)
    }
}
impl Div<NanoSecond> for Currency {
    type Output = Token;
    fn div(self: Currency, rhs: NanoSecond) -> Token {
        Token(self.0 / rhs.0)
    }
}
impl Div<&NanoSecond> for Currency {
    type Output = Token;
    fn div(self: Currency, rhs: &NanoSecond) -> Token {
        Token(self.0 / rhs.0)
    }
}
impl Rem<NanoSecond> for Currency {
    type Output = Token;
    fn rem(self: Currency, rhs: NanoSecond) -> Token {
        Token(self.0 % rhs.0)
    }
}
impl Rem<&NanoSecond> for Currency {
    type Output = Token;
    fn rem(self: Currency, rhs: &NanoSecond) -> Token {
        Token(self.0 % rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_typo_in_npd_constant() {
        assert_eq!(
            NANOSECONDS_PER_DAY,
            Token(1_000_000_000 * 60 * 60 * 24)
        )
    }
    #[test]
    fn nanosecond_mul() {
        for x in 0..64 {
            for y in 0..64 {
                assert_eq!(
                    x * y,
                    (NanoSecond::from(x) * Token::from(y)).0
                )
            }
        }
    }
}
