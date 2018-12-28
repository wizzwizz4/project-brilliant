import unittest
try:
    from . import infinite_auction
except (SystemError, ImportError):
    import __init__ as infinite_auction

# API

# Internals
class Test_winner_and_bid(unittest.TestCase):
    def test_first_come(self):
        # TODO: Add underscores back to the prices (500 -> 5_00)
        winner, bid = infinite_auction._winner_and_bid((
            infinite_auction.Bid( 500 * infinite_auction.NANOSECONDS_PER_DAY,
                                 2500 * infinite_auction.NANOSECONDS_PER_DAY,
                                 9001,  # that's impossible!
                                 "Winner"),
            infinite_auction.Bid( 100 * infinite_auction.NANOSECONDS_PER_DAY,
                                  500 * infinite_auction.NANOSECONDS_PER_DAY,
                                 9002,  # more impossible!
                                 "No chance"),
            infinite_auction.Bid( 500 * infinite_auction.NANOSECONDS_PER_DAY,
                                   10 * infinite_auction.NANOSECONDS_PER_DAY,
                                 9400,  # crazy!
                                 "Sadly not")
        ),                         10 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(winner.id, "Winner")
        self.assertEqual(bid,     500 * infinite_auction.NANOSECONDS_PER_DAY)

    def test_no_bid(self):
        winner, bid = infinite_auction._winner_and_bid((), 2362)
        self.assertEqual(winner, None)
        self.assertEqual(bid, 0)

    def test_one_bid(self):
        winner, bid = infinite_auction._winner_and_bid((
            infinite_auction.Bid( 500 * infinite_auction.NANOSECONDS_PER_DAY,
                                 9000 * infinite_auction.NANOSECONDS_PER_DAY,
                                 3,
                                 "Winner"),
        ),                         10 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(winner.id, "Winner")
        self.assertEqual(bid, 0)

# Constants
class TestConstants(unittest.TestCase):
    def test_NANOSECONDS_PER_DAY(self):
        self.assertEqual(infinite_auction.NANOSECONDS_PER_DAY,
                         1000**3 * 60 * 60 * 24)

if __name__ == "__main__":
    unittest.main()