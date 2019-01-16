"""An implementation of Project Wonderful's Infinite Auction."""

# This implementation is unaffiliated with Project Wonderful and is not
# guaranteed to behave identically to Project Wonderful's implementation.
# For a friendly description of the algorithm, see:
# http:s//web.archive.org/web/20180612112237/
# https://www.projectwonderful.com/abouttheinfiniteauction.php

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
    """Gets the highest bid. In the event of a tie, goes with the first."""
    bid_iterator = iter(bids)
    try:
        highest = next(bid_iterator)  # Priority to the first bid
    except StopIteration:
        return None, 0
    try:
        current = next(bid_iterator)
    except StopIteration:
        return highest, 0
    if current.bid > highest.bid:
        runner_up_bid = highest.bid
        highest = current
    else:
        runner_up_bid = current.bid

    for current in bid_iterator:
        current_bid = current.bid
        if current_bid > highest.bid:
            runner_up_bid = highest.bid
            highest = current
            continue
        if current_bid > runner_up_bid:
            runner_up_bid = current_bid

    # We have to cap the bid to the maximum bid provided.
    bid = min(highest.bid, runner_up_bid + increment)

    return highest, bid

def valid_bids(bids, now: Nanotime=0):
    """Filters bids for valid ones.

    It's best to do this in SQL or something; don't use this function please.
    """
    for bid in bids:
        if bid.bid > 0 and bid.expense_limit > 0 and bid.expiry > now:
            yield bid

def find_end(amount: Currency, limit: Tokens,
             expiry: Nanotime, now: Nanotime) -> (Nanotime, Tokens):
    """Finds the end point of a bid, so a timer can be set."""
    if expiry <= now:
        return None, None
    if amount == 0:
        return None, 0
    broke = (limit // amount) + now
    if broke > expiry:
        return expiry, ((expiry - now) * amount)
    return broke, limit - (limit % amount)
