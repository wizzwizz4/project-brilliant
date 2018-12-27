"""An implementation of Project Wonderful's Infinite Auction."""

# This implementation is unaffiliated with Project Wonderful and is not
# guaranteed to behave identically to Project Wonderful's implementation.
# For a friendly description of the algorithm, see:
# http:s//web.archive.org/web/20180612112237/
# https://www.projectwonderful.com/abouttheinfiniteauction.php

NANOSECONDS_PER_DAY = 86400000000000

class Bid:
    __slots__ = 'bid', 'expense_limit', 'expiry', 'id'

    def __init__(self, bid: int, expense_limit: int, expiry: int, id=None):
        self.bid = bid
        self.expense_limit = expense_limit
        self.expiry = expiry
        self.id = id

def _highest_and_runner_up(bids):
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

    return highest, runner_up_bid

def winning_bid(bids, increment: int) -> (Bid, int, int):
    """Gets the highest bid. In the event of a tie, goes with the first.
    """
    # Get the highest two bids
    winner, bid = _highest_and_runner_up(bids)
    bid = min(winner.bid, bid + increment)  # bid must be bigger, hopefully
