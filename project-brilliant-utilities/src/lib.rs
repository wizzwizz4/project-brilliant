#[macro_use]
extern crate derive_more;

use std::ops::{Mul, Div, Rem};

pub const NANOSECONDS_PER_DAY: Token = Token(86_400_000_000_000);

#[derive(Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign,
         Sub, SubAssign)]
pub struct NanoSecond(u64);
#[derive(Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign,
         Sub, SubAssign)]
pub struct Currency(u64);
#[derive(Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign,
         Sub, SubAssign)]
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
impl Div<Token> for Currency {
    type Output = NanoSecond;
    fn div(self: Currency, rhs: Token) -> NanoSecond {
        NanoSecond(self.0 / rhs.0)
    }
}
impl Div<&Token> for Currency {
    type Output = NanoSecond;
    fn div(self: Currency, rhs: &Token) -> NanoSecond {
        NanoSecond(self.0 / rhs.0)
    }
}
impl Rem<Token> for Currency {
    type Output = NanoSecond;
    fn rem(self: Currency, rhs: Token) -> NanoSecond {
        NanoSecond(self.0 % rhs.0)
    }
}
impl Rem<&Token> for Currency {
    type Output = NanoSecond;
    fn rem(self: Currency, rhs: &Token) -> NanoSecond {
        NanoSecond(self.0 % rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add};

    // Rust: Something's too hard? Butcher it with a macro!
    macro_rules! sparse_to_32 {
        () => {
                   (0..64).step_by(1)
            .chain((64..65536).step_by(13))
            .chain((65536..4_294_967_296).step_by(67_867_967))
        }
    }
    macro_rules! sparse_to_64 {
        () => {
            sparse_to_32!()
            .chain((4_294_967_296..=18_446_744_073_709_551_615).step_by(
                184_467_447_370_955_227
            ))
        }
    }

    #[test]
    fn no_typo_in_npd_constant() {
        assert_eq!(
            NANOSECONDS_PER_DAY,
            Token(1_000_000_000 * 60 * 60 * 24)
        )
    }

    #[test]
    fn test_add_sub() {
        for x in sparse_to_64!().take_while(|x| *x < (u64::max_value() / 2)) {
            for y in sparse_to_64!().take_while(|y| *y <= x) {
                assert_eq!(
                    NanoSecond(x + y),
                    NanoSecond::from(x) + NanoSecond::from(y),
                    "NanoSecond::from({}) + NanoSecond::from({})", x, y
                );
                assert_eq!(
                    NanoSecond(x - y),
                    NanoSecond::from(x) - NanoSecond::from(y),
                    "NanoSecond::from({}) - NanoSecond::from({})", x, y
                );
                let mut z = NanoSecond::from(x);
                z += NanoSecond::from(y);
                assert_eq!(
                    NanoSecond(x + y), z,
                    "NanoSecond::from({}) += NanoSecond::from({})", x, y
                );
                let mut z = NanoSecond::from(x);
                z -= NanoSecond::from(y);
                assert_eq!(
                    NanoSecond(x - y), z,
                    "NanoSecond::from({}) -= NanoSecond::from({})", x, y
                );

                assert_eq!(
                    Currency(x + y),
                    Currency::from(x) + Currency::from(y),
                    "Currency::from({}) + Currency::from({})", x, y
                );
                assert_eq!(
                    Currency(x - y),
                    Currency::from(x) - Currency::from(y),
                    "Currency::from({}) - Currency::from({})", x, y
                );
                let mut z = Currency::from(x);
                z += Currency::from(y);
                assert_eq!(
                    Currency(x + y), z,
                    "Currency::from({}) += Currency::from({})", x, y
                );
                let mut z = Currency::from(x);
                z -= Currency::from(y);
                assert_eq!(
                    Currency(x - y), z,
                    "Currency::from({}) -= Currency::from({})", x, y
                );

                assert_eq!(
                    Token(x + y),
                    Token::from(x) + Token::from(y),
                    "Token::from({}) + Token::from({})", x, y
                );
                assert_eq!(
                    Token(x - y),
                    Token::from(x) - Token::from(y),
                    "Token::from({}) - Token::from({})", x, y
                );
                let mut z = Token::from(x);
                z += Token::from(y);
                assert_eq!(
                    Token(x + y), z,
                    "Token::from({}) += Token::from({})", x, y
                );
                let mut z = Token::from(x);
                z -= Token::from(y);
                assert_eq!(
                    Token(x - y), z,
                    "Token::from({}) -= Token::from({})", x, y
                );
            }
        }
    }

    #[test]
    fn test_mul() {
        // you know what, just test everything at once
        for x in sparse_to_32!() {
            for y in sparse_to_32!() {
                assert_eq!(
                    Currency(x * y),
                    NanoSecond::from(x) * Token::from(y),
                    "NanoSecond::from({}) * Token::from({})", x, y
                );
                assert_eq!(
                    Currency(x * y),
                    NanoSecond::from(x) * &Token::from(y),
                    "NanoSecond::from({}) * &Token::from({})", x, y
                );
                assert_eq!(
                    Currency(x * y),
                    Token::from(x) * NanoSecond::from(y),
                    "Token::from({}) * NanoSecond::from({})", x, y
                );
                assert_eq!(
                    Currency(x * y),
                    Token::from(x) * &NanoSecond::from(y),
                    "Token::from({}) * &NanoSecond::from({})", x, y
                );
            }
        }
    }
    #[test]
    fn test_div_rem() {
        for x in sparse_to_64!() {
            for y in sparse_to_32!().skip(1) {
                assert_eq!(
                    Token(x / y),
                    Currency::from(x) / NanoSecond::from(y),
                    "Currency::from({}) / NanoSecond::from({})", x, y
                );
                assert_eq!(
                    Token(x / y),
                    Currency::from(x) / &NanoSecond::from(y),
                    "Currency::from({}) / &NanoSecond::from({})", x, y
                );
                assert_eq!(
                    Token(x % y),
                    Currency::from(x) % NanoSecond::from(y),
                    "Currency::from({}) % NanoSecond::from({})", x, y
                );
                assert_eq!(
                    Token(x % y),
                    Currency::from(x) % &NanoSecond::from(y),
                    "Currency::from({}) % &NanoSecond::from({})", x, y
                );
                assert_eq!(
                    NanoSecond(x / y),
                    Currency::from(x) / Token::from(y),
                    "Currency::from({}) / Token::from({})", x, y
                );
                assert_eq!(
                    NanoSecond(x / y),
                    Currency::from(x) / &Token::from(y),
                    "Currency::from({}) / &Token::from({})", x, y
                );
                assert_eq!(
                    NanoSecond(x % y),
                    Currency::from(x) % Token::from(y),
                    "Currency::from({}) % Token::from({})", x, y
                );
                assert_eq!(
                    NanoSecond(x % y),
                    Currency::from(x) % &Token::from(y),
                    "Currency::from({}) % &Token::from({})", x, y
                );
            }
        }
    }
}
