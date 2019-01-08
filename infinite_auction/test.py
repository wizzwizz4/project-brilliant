import unittest
try:
    from . import infinite_auction
except (SystemError, ImportError):
    import __init__ as infinite_auction

# API
class Test_winning_bid(unittest.TestCase):
    def test_first_come(self):
        # TODO: Add underscores back to the prices (500 -> 5_00)
        winner, bid = infinite_auction.winning_bid((
            infinite_auction.Bid( 500,
                                 2500 * infinite_auction.NANOSECONDS_PER_DAY,
                                 9001,  # that's impossible!
                                 "Winner"),
            infinite_auction.Bid( 100,
                                  500 * infinite_auction.NANOSECONDS_PER_DAY,
                                 9002,  # more impossible!
                                 "No chance"),
            infinite_auction.Bid( 500,
                                   10 * infinite_auction.NANOSECONDS_PER_DAY,
                                 9400,  # crazy!
                                 "Sadly not")
        ),                         10)
        self.assertEqual(winner.id, "Winner")
        self.assertEqual(bid,     500)

    def test_no_bid(self):
        winner, bid = infinite_auction.winning_bid((), 2362)
        self.assertEqual(winner, None)
        self.assertEqual(bid, 0)

    def test_one_bid(self):
        winner, bid = infinite_auction.winning_bid((
            infinite_auction.Bid( 500,
                                 9000 * infinite_auction.NANOSECONDS_PER_DAY,
                                 3,
                                 "Winner"),
        ),                         10)
        self.assertEqual(winner.id, "Winner")
        self.assertEqual(bid, 0)

class Test_valid_bids(unittest.TestCase):
    def test_static(self):
        iterator = infinite_auction.valid_bids((
            infinite_auction.Bid(   0,
                                 1000 * infinite_auction.NANOSECONDS_PER_DAY,
                                 infinite_auction.NANOSECONDS_PER_DAY + 500,
                                 "Invalid"),
            infinite_auction.Bid(1000,
                                    0 * infinite_auction.NANOSECONDS_PER_DAY,
                                 infinite_auction.NANOSECONDS_PER_DAY + 200,
                                 "Invalid"),
            infinite_auction.Bid( 450,
                                 2600 * infinite_auction.NANOSECONDS_PER_DAY,
                                 infinite_auction.NANOSECONDS_PER_DAY + 800,
                                 "Valid #1"),
            infinite_auction.Bid(6300,
                                  240 * infinite_auction.NANOSECONDS_PER_DAY,
                                 infinite_auction.NANOSECONDS_PER_DAY - 124,
                                 "Invalid"),
            infinite_auction.Bid(8450,
                                 6820 * infinite_auction.NANOSECONDS_PER_DAY,
                                 infinite_auction.NANOSECONDS_PER_DAY + 620,
                                 "Valid #2")
        ), infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual([bid.id for bid in iterator],
                         ["Valid #1", "Valid #2"])

# Constants
class TestConstants(unittest.TestCase):
    def test_NANOSECONDS_PER_DAY(self):
        self.assertEqual(infinite_auction.NANOSECONDS_PER_DAY,
                         1000**3 * 60 * 60 * 24)

if __name__ == "__main__":
    unittest.main()
