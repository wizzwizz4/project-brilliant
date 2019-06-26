//! An implementation of Project Wonderful's Infinite Auction.
//!
//! This implementation is unaffiliated with Project Wonderful and is not
//! guaranteed to behave identically to Project Wonderful's implementation.
//! For a friendly description of the algorithm, see
//! [www.projectwonderful.com/abouttheinfiniteauction.php](https://web.archive.org/web/20180612112237/https://www.projectwonderful.com/abouttheinfiniteauction.php)

use project_brilliant_utilities::{
    NanoSecond, Currency, Token,
    NANOSECONDS_PER_DAY
};

use std::cmp::min;

pub struct Bid<T: Copy> {
    bid: Currency,
    expense_limit: Token,
    expiry: NanoSecond,
    data: T
}

pub fn winning_bids<'a, T: Copy>(
    mut bids: Vec<&'a mut Bid<T>>,
    increment: Currency
) -> Vec<(&'a mut Bid<T>, Currency)> {
    bids.sort_by(|a, b| b.bid.cmp(&a.bid));
    let bids = bids;

    let mut index = bids.len();
    let mut output = Vec::with_capacity(index);
    let mut to_beat = Currency::from(0);
    for bid in bids.into_iter().rev() {
        index -= 1;
        let bid_amount = bid.bid;
        output[index] = (bid, min(bid_amount, to_beat));
        to_beat = bid_amount + increment;
    }
    assert_eq!(index, 0, "Why can't you just do things normally?");
    output
}

/// Filter bids for valid ones.
///
/// The three checks this performs are:
///
///   * `bid`[`.bid`]()` > `[`Currency`]()`(0)`
///   * `bid`[`.expense_limit`]()` > `[`Token`]()`(0)`
///   * `bid`[`.expiry`]()` > now`
///
/// These checks are better implemented in SQL (or something), since
/// this function is rather inefficient. Nevertheless, since other
/// functions expect to recieve only valid bids, this function may
/// occasionally be necessary.
#[deprecated(note="Refactor to avoid using this function.")]
pub fn valid_bids<'a, T: Copy>(
    bids: Vec<&'a Bid<T>>,
    now: NanoSecond
) -> Vec<&'a Bid<T>> {
    bids.into_iter().filter(|bid| bid.bid    >     Currency::from(0)
                               && bid.expense_limit > Token::from(0)
                               && bid.expiry > now).collect()
}

/// Find the end point of a bid, so a timer can be set.
/// (Returns the time the bid ends, and the number of tokens spent.)
///
/// **WARNING:** Assumes that this is the only bid; ensure that expiry is the
/// minimum of the expiry date of the bid and the amount's validity. A naïve
/// use would be to take the minimum of all bids' expiry dates, but this is
/// suboptimal.
fn find_end(
    amount: Currency, limit: Token,
    expiry: NanoSecond, now: NanoSecond
) -> (NanoSecond, Token) {
    if expiry <= now {
        panic!(
            "Expiry date ({:?}) in the past (before {:?})! Bid invalid.",
                          expiry,                   now
        );
    }
    if amount == Currency::from(0) {
        return (expiry, Token::from(0));
    }
    let broke = (limit / amount) + now;
    return (broke, limit / amount * amount);
}

fn run_auction<T: Copy>(
    mut bids: Vec<Bid<T>>,
    increment: Currency, mut now: NanoSecond
) -> Vec<(T, NanoSecond, Token)> {
    let mut output = Vec::with_capacity(bids.len());  // not max!
    'outer: while !bids.is_empty() {
        let mut candidates = winning_bids(
            bids.iter_mut().collect(),
            increment
        );
        let mut index: usize = 0;
        let bid_amount;
        let candidates_len = candidates.len();
        loop {
            if index >= candidates_len {break 'outer;}
            let (candidate, candidate_amount) = &candidates[index];
            if *candidate_amount * NanoSecond::from(1)
            <= candidate.expense_limit {
                // it has a chance of being able to pay
                bid_amount = *candidate_amount;
                break;
            }
            index += 1;
        };

        let part_off = index + 1;
        let (part_a, part_b) = candidates.split_at_mut(part_off);
        let bid = &mut part_a[index].0;

        let mut expiry;
        loop {
            if index >= candidates_len {
                expiry = bid.expiry;
                break;
            }
            let (candidate, candidate_amount) = &part_b[index - part_off];
            if *candidate_amount * NanoSecond::from(1)
            <= candidate.expense_limit {
                expiry = min(candidate.expiry, bid.expiry);
                break;
            }
        }
        let expiry = expiry;

        let end = find_end(bid_amount, bid.expense_limit,
                           expiry, now);
        now = end.0;
        let spent = end.1;
        assert!(bid_amount == Currency::from(0)
             || spent > Token::from(0),
                "Stuck in an infinite loop!");
        bid.expense_limit -= spent;

        output.push((bid.data, now, spent));

        bids.retain(|bid| bid.expiry > now
                       && bid.expense_limit > Token::from(0));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
