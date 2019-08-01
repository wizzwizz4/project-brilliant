#[macro_use]
extern crate derive_more;

use std::ops::{Mul, Div, Rem};

pub const SECONDS_PER_DAY: Second = Second(86_400);

type IntegerType = u64;

#[derive(Copy, Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord,
         Add, AddAssign, Sub, SubAssign)]
pub struct Second(IntegerType);
#[derive(Copy, Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord,
         Add, AddAssign, Sub, SubAssign)]
pub struct Currency(IntegerType);
#[derive(Copy, Clone, Debug, From, PartialEq, Eq, PartialOrd, Ord,
         Add, AddAssign, Sub, SubAssign)]
pub struct Token(IntegerType);

impl Mul<Currency> for Second {
    type Output = Token;
    fn mul(self: Second, rhs: Currency) -> Token {
        Token(self.0 * rhs.0)
    }
}
impl Mul<&Currency> for Second {
    type Output = Token;
    fn mul(self: Second, rhs: &Currency) -> Token {
        Token(self.0 * rhs.0)
    }
}
impl Mul<Second> for Currency {
    type Output = Token;
    fn mul(self: Currency, rhs: Second) -> Token {
        Token(self.0 * rhs.0)
    }
}
impl Mul<&Second> for Currency {
    type Output = Token;
    fn mul(self: Currency, rhs: &Second) -> Token {
        Token(self.0 * rhs.0)
    }
}
impl Div<Second> for Token {
    type Output = Currency;
    fn div(self: Token, rhs: Second) -> Currency {
        Currency(self.0 / rhs.0)
    }
}
impl Div<&Second> for Token {
    type Output = Currency;
    fn div(self: Token, rhs: &Second) -> Currency {
        Currency(self.0 / rhs.0)
    }
}
impl Rem<Second> for Token {
    type Output = Currency;
    fn rem(self: Token, rhs: Second) -> Currency {
        Currency(self.0 % rhs.0)
    }
}
impl Rem<&Second> for Token {
    type Output = Currency;
    fn rem(self: Token, rhs: &Second) -> Currency {
        Currency(self.0 % rhs.0)
    }
}
impl Div<Currency> for Token {
    type Output = Second;
    fn div(self: Token, rhs: Currency) -> Second {
        Second(self.0 / rhs.0)
    }
}
impl Div<&Currency> for Token {
    type Output = Second;
    fn div(self: Token, rhs: &Currency) -> Second {
        Second(self.0 / rhs.0)
    }
}
impl Rem<Currency> for Token {
    type Output = Second;
    fn rem(self: Token, rhs: Currency) -> Second {
        Second(self.0 % rhs.0)
    }
}
impl Rem<&Currency> for Token {
    type Output = Second;
    fn rem(self: Token, rhs: &Currency) -> Second {
        Second(self.0 % rhs.0)
    }
}
impl Mul<IntegerType> for Second {
    type Output = Second;
    fn mul(self: Second, rhs: IntegerType) -> Second {
        Second(self.0 * rhs)
    }
}
impl Mul<Second> for IntegerType {
    type Output = Second;
    fn mul(self: IntegerType, rhs: Second) -> Second {
        Second(self * rhs.0)
    }
}
impl Mul<IntegerType> for Currency {
    type Output = Currency;
    fn mul(self: Currency, rhs: IntegerType) -> Currency {
        Currency(self.0 * rhs)
    }
}
impl Mul<Currency> for IntegerType {
    type Output = Currency;
    fn mul(self: IntegerType, rhs: Currency) -> Currency {
        Currency(self * rhs.0)
    }
}
impl Mul<IntegerType> for Token {
    type Output = Token;
    fn mul(self: Token, rhs: IntegerType) -> Token {
        Token(self.0 * rhs)
    }
}
impl Mul<Token> for IntegerType {
    type Output = Token;
    fn mul(self: IntegerType, rhs: Token) -> Token {
        Token(self * rhs.0)
    }
}
impl Div<IntegerType> for Second {
    type Output = Second;
    fn div(self: Second, rhs: IntegerType) -> Second {
        Second(self.0 / rhs)
    }
}
impl Rem<IntegerType> for Second {
    type Output = Second;
    fn rem(self: Second, rhs: IntegerType) -> Second {
        Second(self.0 % rhs)
    }
}
impl Div<IntegerType> for Currency {
    type Output = Currency;
    fn div(self: Currency, rhs: IntegerType) -> Currency {
        Currency(self.0 / rhs)
    }
}
impl Rem<IntegerType> for Currency {
    type Output = Currency;
    fn rem(self: Currency, rhs: IntegerType) -> Currency {
        Currency(self.0 % rhs)
    }
}
impl Div<IntegerType> for Token {
    type Output = Token;
    fn div(self: Token, rhs: IntegerType) -> Token {
        Token(self.0 / rhs)
    }
}
impl Rem<IntegerType> for Token {
    type Output = Token;
    fn rem(self: Token, rhs: IntegerType) -> Token {
        Token(self.0 % rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // I'm bored of typing stuff.
    // TODO: Refactor all tests to be as lazy as this.
    // WARN: This will be effort. Effort for laziness.
    macro_rules! comparison {
        ('commutative $T:tt $x:tt $op:tt $y:tt) => {
            comparison!($T $x $op $y);
            assert_eq!(
                $T($x $op $y),
                $x $op $T::from($y),
                concat!(
                    "{} ",
                    stringify!($op),
                    " ",
                    stringify!($T),
                    "::from({})"
                ), $x, $y
            )
        };
        ($T:tt $x:tt $op:tt $y:tt) => {
            assert_eq!(
                $T($x $op $y),
                $T::from($x) $op $y,
                concat!(
                    stringify!($T),
                    "::from({}) ",
                    stringify!($op),
                    " {}"
                ), $x, $y
            )
        }
    }

    #[test]
    fn no_typo_in_npd_constant() {
        assert_eq!(SECONDS_PER_DAY, Second(60 * 60 * 24))
    }

    #[test]
    fn test_add_sub() {
        for x in sparse_to_64!().take_while(|x| *x < (u64::max_value() / 2)) {
            for y in sparse_to_64!().take_while(|y| *y <= x) {
                assert_eq!(
                    Second(x + y),
                    Second::from(x) + Second::from(y),
                    "Second::from({}) + Second::from({})", x, y
                );
                assert_eq!(
                    Second(x - y),
                    Second::from(x) - Second::from(y),
                    "Second::from({}) - Second::from({})", x, y
                );
                let mut z = Second::from(x);
                z += Second::from(y);
                assert_eq!(
                    Second(x + y), z,
                    "Second::from({}) += Second::from({})", x, y
                );
                let mut z = Second::from(x);
                z -= Second::from(y);
                assert_eq!(
                    Second(x - y), z,
                    "Second::from({}) -= Second::from({})", x, y
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
                    Token(x * y),
                    Second::from(x) * Currency::from(y),
                    "Second::from({}) * Currency::from({})", x, y
                );
                assert_eq!(
                    Token(x * y),
                    Second::from(x) * &Currency::from(y),
                    "Second::from({}) * &Currency::from({})", x, y
                );
                assert_eq!(
                    Token(x * y),
                    Currency::from(x) * Second::from(y),
                    "Currency::from({}) * Second::from({})", x, y
                );
                assert_eq!(
                    Token(x * y),
                    Currency::from(x) * &Second::from(y),
                    "Currency::from({}) * &Second::from({})", x, y
                );
                comparison!('commutative Second x * y);
                comparison!('commutative Currency x * y);
                comparison!('commutative Token x * y);
            }
        }
    }
    #[test]
    fn test_div_rem() {
        for x in sparse_to_64!() {
            for y in sparse_to_32!().skip(1) {
                assert_eq!(
                    Currency(x / y),
                    Token::from(x) / Second::from(y),
                    "Token::from({}) / Second::from({})", x, y
                );
                assert_eq!(
                    Currency(x / y),
                    Token::from(x) / &Second::from(y),
                    "Token::from({}) / &Second::from({})", x, y
                );
                assert_eq!(
                    Currency(x % y),
                    Token::from(x) % Second::from(y),
                    "Token::from({}) % Second::from({})", x, y
                );
                assert_eq!(
                    Currency(x % y),
                    Token::from(x) % &Second::from(y),
                    "Token::from({}) % &Second::from({})", x, y
                );
                assert_eq!(
                    Second(x / y),
                    Token::from(x) / Currency::from(y),
                    "Token::from({}) / Currency::from({})", x, y
                );
                assert_eq!(
                    Second(x / y),
                    Token::from(x) / &Currency::from(y),
                    "Token::from({}) / &Currency::from({})", x, y
                );
                assert_eq!(
                    Second(x % y),
                    Token::from(x) % Currency::from(y),
                    "Token::from({}) % Currency::from({})", x, y
                );
                assert_eq!(
                    Second(x % y),
                    Token::from(x) % &Currency::from(y),
                    "Token::from({}) % &Currency::from({})", x, y
                );
                comparison!(Second x / y);
                comparison!(Second x % y);
                comparison!(Currency x / y);
                comparison!(Currency x % y);
                comparison!(Token x / y);
                comparison!(Token x % y);
            }
        }
    }
}
