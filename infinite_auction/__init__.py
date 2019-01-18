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

def valid_bids(bids, now: Nanotime=0) -> [Bid]:
    """Filter bids for valid ones.

    It's best to do this in SQL or something; don't use this function please.
    """
    for bid in bids:
        if bid.bid > 0 and bid.expense_limit > 0 and bid.expiry > now:
            yield bid

def find_end(amount: Currency, limit: Tokens,
             expiry: Nanotime, now: Nanotime) -> (Nanotime, Tokens):
    """Find the end point of a bid, so a timer can be set."""
    if expiry <= now:
        raise ValueError("Expiry date in the past! Bid invalid.")
    if amount == 0:
        return expiry, 0
    broke = (limit // amount) + now
    if broke > expiry:
        return expiry, ((expiry - now) * amount)
    return broke, limit - (limit % amount)

def run_auction(bids, increment: Currency,
                now: Nanotime) -> [(Bid, Nanotime, Tokens)]:
    """Run an auction and yield (current bid, when it ends, tokens spent)."""
    # Doesn't work! Flag when each bid has expired, and recalculate which win.
    for bid, bid_amount_c in winning_bids(bids, increment):
        now, spent_t = find_end(bid_amount_c, bid.expense_limit,
                                bid.expiry, now)
        yield bid, now, spent_t
