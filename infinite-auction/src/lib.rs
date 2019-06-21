use project_brilliant_utilities::{
    NanoSecond, Currency, Token,
    NANOSECONDS_PER_DAY
};

pub struct Bid<'a, T> {
    bid: Currency,
    expense_limit: Token,
    expiry: NanoSecond,
    data: &'a T
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
