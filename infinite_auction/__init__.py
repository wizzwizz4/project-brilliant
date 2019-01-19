"""An implementation of Project Wonderful's Infinite Auction."""

# This implementation is unaffiliated with Project Wonderful and is not
# guaranteed to behave identically to Project Wonderful's implementation.
# For a friendly description of the algorithm, see:
# https://web.archive.org/web/20180612112237/
# https://www.projectwonderful.com/abouttheinfiniteauction.php

from collections import deque

NANOSECONDS_PER_DAY = 86400000000000
Nanotime = int
Currency = int
Tokens = int

class Bid:
    __slots__ = 'bid', 'expense_limit', 'expiry', 'id'

    def __init__(self, bid: Currency, expense_limit: Tokens,
                 expiry: Nanotime, id=None):
        self.bid = bid
        self.expense_limit = expense_limit
        self.expiry = expiry
        self.id = id

def winning_bid(bids, increment: Currency) -> (Bid, Currency):
    """Get the highest bid. In the event of a tie, goes with the first."""
    try:
        return next(winning_bids(bids, increment))
    except StopIteration:
        return None, 0

def winning_bids(bids, increment: Currency) -> [(Bid, Currency)]:
    bids = sorted(enumerate(bids), key=lambda a: (a[1].bid, -a[0]))

    output = deque()
    to_beat = 0
    for i, bid in bids:
        bid_amount = bid.bid
        output.appendleft((bid, min(bid_amount, to_beat)))
        to_beat = bid_amount + increment
    yield from output

def valid_bids(bids: [Bid], now: Nanotime=0) -> [Bid]:
    """Filter bids for valid ones.

    It's best to do this in SQL or something; don't use this function please.
    """
    for bid in bids:
        if bid.bid > 0 and bid.expense_limit > 0 and bid.expiry > now:
            yield bid

def find_end(amount: Currency, limit: Tokens,
             expiry: Nanotime, now: Nanotime) -> (Nanotime, Tokens):
    """Find the end point of a bid, so a timer can be set.

    WARNING: Assumes that this is the only bid; ensure that expiry is the
    minimum of the expiry date of the bid and the amount's validity. A naïve
    use would be to take the minimum of all bids' expiry dates, but this is
    suboptimal.
    """
    if expiry <= now:
        raise ValueError("Expiry date in the past! Bid invalid.")
    if amount == 0:
        return expiry, 0
    broke = (limit // amount) + now
    if broke > expiry:
        return expiry, ((expiry - now) * amount)
    return broke, limit - (limit % amount)

def run_auction(bids: [Bid], increment: Currency,
                now: Nanotime) -> [(Bid, Nanotime, Tokens)]:
    """Run an auction and yield (current bid, when it ends, tokens spent)."""
    bids = list(bids)
    expense_limits = [bid.expense_limit for bid in bids]
    while bids:
        candidates = winning_bids(bids, increment)
        while True:
            try:
                candidate, candidate_amount_c = next(candidates)
            except StopIteration as e:
                raise AssertionError(
                    "There are literally no candidates."
                ) from e
            index = bids.index(candidate)
            if candidate_amount_c <= expense_limits[index]:
                # It has a chance of being able to pay
                bid = candidate
                bid_amount_c = candidate_amount_c
                break
        candidate_index = None
        while True:
            try:
                candidate, candidate_amount_c = next(candidates)
            except StopIteration as e:
                expiry = bid.expiry
                break
            candidate_index = bids.index(candidate)
            if candidate_amount_c <= expense_limits[candidate_index]:
                expiry = min(candidate.expiry, bid.expiry)
                break
        del candidate, candidate_amount_c, candidate_index

        now, spent_t = find_end(bid_amount_c, expense_limits[index],
                                expiry, now)
        assert bid_amount_c == 0 or spent_t > 0, "Stuck in an infinite loop!"
        expense_limits[index] -= spent_t

        yield bid, now, spent_t

        i = 0
        while i < len(bids):
            if bids[i].expiry <= now:
                del bids[i]
                del expense_limits[i]
            else:
                i += 1
