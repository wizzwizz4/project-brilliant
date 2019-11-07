//! An implementation of Project Wonderful's Infinite Auction.
//!
//! This implementation is unaffiliated with Project Wonderful and is not
//! guaranteed to behave identically to Project Wonderful's implementation.
//! For a friendly description of the algorithm, see
//! [www.projectwonderful.com/abouttheinfiniteauction.php](https://web.archive.org/web/20180612112237/https://www.projectwonderful.com/abouttheinfiniteauction.php)

use project_brilliant_utilities::{
    Second, Currency, Token,
    SECONDS_PER_DAY
};

use std::cmp::{min, max};

#[derive(Debug)]
pub struct Bid<T: Copy> {
    bid: Currency,
    expense_limit: Token,
    expiry: Second,
    data: T
}

fn winning_bid<'a, T: Copy>(
    bids: Vec<&'a mut Bid<T>>,
    increment: Currency, min_bid: Currency
) -> Option<(&'a mut Bid<T>, Currency, Second)> {
    let mut bids = bids.into_iter().filter(|x| x.bid >= min_bid)
        .collect::<Vec<&'a mut Bid<T>>>();
    bids.sort_by(|a, b| b.bid.cmp(&a.bid));

    /////////////////////////////////////////////////////

    // let mut index: usize = 0;
    // let candidates_len = candidates.len();
    // let bid_amount = loop {
    //     if index >= candidates_len {return None;}
    //     let (candidate, candidate_amount) = &candidates[index];
    //     if *candidate_amount * Second::from(1)
    //     <= candidate.expense_limit {
    //         // it has a chance of being able to pay
    //         break *candidate_amount;
    //     }
    //     index += 1;
    // };
    //
    // let part_off = index + 1;
    // let (part_a, part_b) = candidates.split_at_mut(part_off);
    // let bid = &mut part_a[index].0;
    // index += 1;
    //
    // let expiry = loop {
    //     if index >= candidates_len {
    //         break bid.expiry;
    //     }
    //     let (candidate, candidate_amount) = &part_b[index - part_off];
    //     if *candidate_amount * Second::from(1)
    //     <= candidate.expense_limit {
    //         break min(candidate.expiry, bid.expiry);
    //     }
    // };

    ////////////////////////////////////////////////////

    let mut to_beat = min_bid;

    let split_index = {
        let mut index = bids.len();
        loop {
            if index == 0 {
                return None;
            }
            index -= 1;
            let bid = &bids[index];
            if min_bid * Second::from(1) <= bid.expense_limit {
                break index;
            }
        }
    };
    bids.truncate(split_index + 1);
    let mut winner: &'a mut Bid<T> = bids.remove(split_index);
    let mut bid_amount = min(winner.bid, to_beat);
    to_beat = winner.bid + increment;
    let mut expiry = winner.expiry;

    for bid in bids.into_iter().rev() {
        if bid.expense_limit >= to_beat * Second::from(1) {
            expiry = winner.expiry;
            bid_amount = min(bid.bid, to_beat);
            to_beat = bid.bid + increment;
            winner = bid;
        }
    }

    Some((winner, bid_amount, expiry))
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
    bids: &'a [Bid<T>],
    now: Second
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
    expiry: Second, now: Second
) -> (Second, Token) {
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
    if broke > expiry {
        return (expiry, (expiry - now) * amount)
    }
    return (broke, limit / amount * amount);
}

pub fn run_auction<T: Copy>(
    mut bids: Vec<Bid<T>>, increment: Currency,
    min_bid: Currency, mut now: Second
) -> Vec<(T, Second, Token)> {
    // Any less than this and it hasn't a chance.
    let min_tokens = max(min_bid * Second::from(1), Token::from(1));

    // Initial filtration.
    bids.retain(|bid| bid.expiry > now
                   && bid.bid >= min_bid
                   && bid.expense_limit >= min_tokens);

    let mut output = Vec::with_capacity(bids.len());  // not max!
    'outer: while !bids.is_empty() {
        let (mut bid, bid_amount, expiry) = winning_bid(
            bids.iter_mut().collect(),
            increment, min_bid
        ).unwrap();

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
                       && bid.expense_limit >= min_tokens);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_almost_eq {
        ($left:expr, $right:expr, within $bound:expr) => ({
            match (&($left), &($right), &($bound)) {
                (left, right, bound) => {
                    if *left >= *right && *left - *right >= *bound
                    || *right >= *left && *right - *left >= *bound {
                         panic!(r#"assertion failed: \
                         `(left =~= right within bound)`
  left: `{:?}`,
 right: `{:?}`,
 bound: `{:?}`"#, left, right, bound);
                     }
                }
            }
        });
    }

    #[test]
    fn winning_bid_first_come() {
        let mut bids = vec![
            Bid {
                bid:           Currency::from( 5_00),
                expense_limit: Currency::from(25_00) * SECONDS_PER_DAY,
                expiry: Second::from(9001),  // that's impossible!
                data: "Winner"
            },
            Bid {
                bid:           Currency::from( 1_00),
                expense_limit: Currency::from( 5_00) * SECONDS_PER_DAY,
                expiry: Second::from(9002),  // more impossible!
                data: "No chance"
            },
            Bid {
                bid:           Currency::from( 5_00),
                expense_limit: Currency::from(   10) * SECONDS_PER_DAY,
                expiry: Second::from(9400),  // crazy!
                data: "Sadly not"
            }
        ];
        let (winner, bid, expiry) = winning_bid(
            bids.iter_mut().collect(),
            Currency::from(                      10),
            Currency::from(                       0)
        ).unwrap();
        assert_eq!(winner.data, "Winner");
        assert_eq!(bid, Currency::from(5_00));
    }
    #[test]
    fn winning_bid_no_bid() {
        assert!(winning_bid::<()>(
            vec![],
            Currency::from(2362),
            Currency::from(311)
        ).is_none());
    }
    #[test]
    fn winning_bid_one_bid() {
        let mut bids = vec![
            Bid {
                bid:           Currency::from( 5_00),
                expense_limit: Currency::from(90_00) * SECONDS_PER_DAY,
                expiry: Second::from(3),
                data: "Winner"
            }
        ];
        let (winner, bid, expiry) = winning_bid(
            bids.iter_mut().collect(),
            Currency::from(                      10),
            Currency::from(                       0)
        ).unwrap();
        assert_eq!(winner.data, "Winner");
        assert_eq!(bid, Currency::from(0))
    }

    #[test]
    fn valid_bids_static() {
        let bids = [
            Bid {
                bid:           Currency::from( 0_00),
                expense_limit: Currency::from( 1_00) * SECONDS_PER_DAY,
                expiry: SECONDS_PER_DAY + Second::from(500),
                data: "Invalid"
            },
            Bid {
                bid:           Currency::from(10_00),
                expense_limit: Token::from(0),
                expiry: SECONDS_PER_DAY + Second::from(200),
                data: "Invalid"
            },
            Bid {
                bid:           Currency::from( 4_50),
                expense_limit: Currency::from(26_00) * SECONDS_PER_DAY,
                expiry: SECONDS_PER_DAY + Second::from(800),
                data: "Valid #1"
            },
            Bid {
                bid:           Currency::from(63_00),
                expense_limit: Currency::from( 2_40) * SECONDS_PER_DAY,
                expiry: SECONDS_PER_DAY - Second::from(124),
                data: "Invalid"
            },
            Bid {
                bid:           Currency::from(84_50),
                expense_limit: Currency::from(68_20) * SECONDS_PER_DAY,
                expiry: SECONDS_PER_DAY + Second::from(620),
                data: "Valid #2"
            }
        ];
        #[allow(deprecated)]
        let output = valid_bids(&bids, SECONDS_PER_DAY);
        assert_eq!(
            output.into_iter().map(|bid| bid.data).collect::<Vec<&str>>(),
            vec!["Valid #1", "Valid #2"]
        );
    }

    #[test]
    fn find_end_valid() {
        for amount in (0..1_000).step_by(73) {
            for limit in (0..300_000_000_000_000).step_by(772_757_382_975) {
                for now in (0..3_000_000_000_000).step_by(548_795_182_242) {
                    for expiry in (now+1..3_000_000_000_000)
                    .step_by(5_647_289_173_652) {
                        let amount = Currency::from(amount);
                        let limit = Token::from(limit);
                        let now = Second::from(now);
                        let expiry = Second::from(expiry);
                        let (finish, spent) = find_end(amount, limit,
                                                       expiry, now);
                        // Sanity
                        assert!(finish <= expiry,
                            "{:?} > {:?}", finish, expiry);
                        assert!(finish >= now,
                            "{:?} < {:?}", finish, now);

                        // Validity
                        assert!(amount * (finish - now) <= limit,
                            "{:?} * ({:?} - {:?}) > {:?}",
                            amount, finish, now, limit);
                        assert!(spent <= limit,
                            "{:?} > {:?}", spent, limit);

                        if finish < expiry {
                            // Didn't give up early for nothing
                            // Not possible to have spent more
                            assert!(
                                limit - spent
                              < amount * Second::from(1),
                                "{:?} - {:?} < {:?}; money unspent",
                                limit, spent, amount
                            );
                        } else {
                            // Spent the right amount of money
                            assert!(spent / (expiry - now) == amount,
                                "{:?} / ({:?} - {:?}) != {:?}",
                                spent, expiry, now, amount);
                            assert!(
                                spent % (expiry - now)
                             == Currency::from(0),
                                "{:?} % ({:?} - {:?}) != 0",
                                spent, expiry, now
                            );
                        }
                    }
                }
            }
        }
    }

    /// Alice starts out and bids a maximum of $5 a day for a week.
    /// Since she's the only bidder, her bid starts out at $0.
    /// Free advertising!
    #[test]
    fn run_auction_alice_bids() {
        let bids = vec![
            Bid {
                bid:           Currency::from(  5_00),  // $5
                expense_limit: Token::from(42),  // unspecified (should default
                                                 // to 500 * 7 * N_PER_DAY)
                expiry: 7 * SECONDS_PER_DAY,
                data: "Alice"
            }
        ];
        let auction = run_auction(
            bids,
            Currency::from(                     0_10),  // 10¢
            Currency::from(                     0_00),
            Second::from(0)                             // t=0
        );
        assert_eq!(auction.len(), 1);

        // Alice starts out…
        let (alice, expiry, spent) = auction[0];
        assert_eq!(alice, "Alice");
        assert_eq!(expiry, 7 * SECONDS_PER_DAY);
        assert_eq!(spent, Token::from(0));
    }

    /// Partario wants in! He sees Alice's bid of $0 and decides
    /// he'll outbid her with a bid of $1 a day for one day.
    /// He places his bid, but Alice's bid automatically outbids him.
    ///
    /// A day later, Partario's bid expires, and Alice's bid
    /// automatically drops back down to $0.
    /// Free advertising!
    #[test]
    fn run_auction_partario_outbid() {
        let bids = vec![
            Bid {
                bid:           Currency::from(  5_00),  // $5
                expense_limit: Currency::from(  1_10)   // $1.10
                             * SECONDS_PER_DAY
                             + Token::from(42),
                expiry: 7 * SECONDS_PER_DAY,
                data: "Alice"
            },
            Bid {
                bid:           Currency::from(  1_00),  // $1
                expense_limit: Token::from(42),  // 42 == unspecified
                expiry: 1 * SECONDS_PER_DAY,
                data: "Partario"
            }
        ];
        let auction = run_auction(
            bids,
            Currency::from(                       10),  // 10¢
            Currency::from(                        0),
            Second::from(0)                             // t=0
        );
        assert_eq!(auction.len(), 2);

        // Partario wants in!…
        let (alice, expiry, spent) = auction[0];
        assert_eq!(alice, "Alice");
        assert_eq!(expiry, 1 * SECONDS_PER_DAY);
        assert_eq!(spent, Currency::from(       1_10) * SECONDS_PER_DAY);

        // A day later…
        let (alice, expiry, spent) = auction[1];
        assert_eq!(alice, "Alice");
        assert_eq!(expiry, 7 * SECONDS_PER_DAY);
        assert_eq!(spent, Token::from(0));
    }

    /// Partario's back, and this time he decides to bid $100 a day
    /// for a week, with an expense limit of $1. Alice is outbid!
    /// Notice that Partario's bid only goes as high as it needs to
    /// in order to defeat Alice.
    ///
    /// Since Partario's bid is pretty big, but his expense limit is pretty
    /// small, the $1 expense limit is very quickly reached. His bid was
    /// charged at $5.10/day to a maximum of $1, so he got about five hours of
    /// display time ($5.10/day is about 20 cents an hour) before
    /// that $1 limit was reached.
    ///
    /// His bid expires, and Alice's takes over again at $0.
    /// Free advertising!
    #[test]
    fn run_auction_partarios_revenge() {
        let bids = vec![
            Bid {
                bid:           Currency::from(  5_00),  // $5
                expense_limit: Token::from(42),
                expiry: 7 * SECONDS_PER_DAY,
                data: "Alice"
            },
            Bid {
                bid:           Currency::from(100_00),  // $100
                expense_limit: Currency::from(  1_00)   // $1
                             * SECONDS_PER_DAY,
                expiry: 1 * SECONDS_PER_DAY,
                data: "Partario"
            }
        ];
        let auction = run_auction(
            bids,
            Currency::from(                       10),  // 10¢
            Currency::from(                        0),
            Second::from(0)
        );
        assert_eq!(auction.len(), 2);

        // Since Partario's bid…
        let (partario, expiry, spent) = auction[0];
        assert_eq!(partario, "Partario");
        assert_almost_eq!(expiry, SECONDS_PER_DAY / 24 * 5,
            within SECONDS_PER_DAY / 24);
        assert_almost_eq!(spent, Currency::from(1_00)   // $1
                               * SECONDS_PER_DAY,
            within Token::from(10_000));

        // His bid expires…
        let (alice, expiry, spent) = auction[1];
        assert_eq!(alice, "Alice");
        assert_eq!(expiry, 7 * SECONDS_PER_DAY);
        assert_eq!(spent, Token::from(0));
    }

    /// Partino also decides to place a second bid, this time of $1 a day
    /// for a week. This way, if his more expensive bid expires, he'll still
    /// have a chance of his ad being shown at a rate he likes! This new bid
    /// is outbid, but his older bid of $100 (max) is still the high bidder,
    /// so his ad is being shown.
    ///
    /// Since Partario's bid is pretty big, but his expense limit is pretty
    /// small, the $1 expense limit is very quickly reached. His bid expires,
    /// and once again Alice is the highest bidder. Partario's second bid of
    /// $1 a day forces Alice's bid up a bit.
    ///
    /// Finally, a week after she placed it, Alice's bid expiry date is
    /// reached, and it expires. Partario's bid of $1 a day is now the high
    /// bidder! Nobody else has bid, so: free advertising!
    #[test]
    fn run_auction_partarios_revenge_2() {
        let bids = vec![
            Bid {
                bid:           Currency::from(  5_00),  // $5
                expense_limit: Currency::from(  5_00)
                             * 7 * SECONDS_PER_DAY
                             + Token::from(42),
                expiry: 7 * SECONDS_PER_DAY,
                data: "Alice"
            },
            Bid {
                bid:           Currency::from(100_00),  // $100
                expense_limit: Currency::from(  1_00)   // $1
                             * SECONDS_PER_DAY,
                expiry: 8 * SECONDS_PER_DAY,  // started at 1d
                data: "Partario"
            },
            Bid {
                bid:           Currency::from(  1_00),  // $1
                expense_limit: Token::from(42),
                expiry: 8 * SECONDS_PER_DAY,
                data: "Partario2"
            }
        ];
        let auction = run_auction(
            bids,
            Currency::from(                       10),  // 10¢
            Currency::from(                        0),
            SECONDS_PER_DAY                             // t=1d
        );
        assert_eq!(auction.len(), 3);

        // Partario also decides…
        let (partario, expiry, spent) = auction[0];
        assert_eq!(partario, "Partario");
        assert_almost_eq!(expiry, SECONDS_PER_DAY / 24 * (24 + 5),
            within SECONDS_PER_DAY / 24);
        assert_almost_eq!(spent, Currency::from(1_00)   // $1
                               * SECONDS_PER_DAY,
            within Token::from(10_000));

        // Since Partario's bid…
        assert_eq!(
            auction[1],
            (
                "Alice",
                7 * SECONDS_PER_DAY,
                (7 * SECONDS_PER_DAY - expiry)
                               * Currency::from(1_10)   // $1
            )
        );

        // Finally, a week…
        assert_eq!(
            auction[2],
            ("Partario2", 8 * SECONDS_PER_DAY, Token::from(0))
        );
    }

    #[test]
    fn run_auction_none_can_bid() {
        let bids = vec![
            Bid {
                bid:           Currency::from(  5_00),
                expense_limit: Token::from(     4_00),
                expiry: Second::from(10_000),
                data: "Can't pay."
            },
            Bid {
                bid:           Currency::from(  5_00),
                expense_limit: Token::from(     6_00),
                expiry: Second::from(10_000),
                data: "Can only pay once."
            }
        ];
        let auction = run_auction(
            bids,
            Currency::from(                       10),
            Currency::from(                     4_60),
            Second::from(0)
        );
        assert_eq!(auction.len(), 1);

        assert_eq!(
            auction[0],
            ("Can only pay once.", Second::from(1), Token::from(460))
        );
    }
}
