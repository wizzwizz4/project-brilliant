import unittest
from math import inf
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

class Test_find_end(unittest.TestCase):
    def test_valid(self):
        for amount in range(0, 1000, 73):
          for limit in range(0, 300000000000000, 772757382975):
            for now in range(0, 3000000000000, 548795182242):
              for expiry in range(now+1, 3000000000000, 5647289173652):
                with unittest.TestCase.subTest(self,
                                              (amount, limit, expiry, now)):
                    finish, spent = infinite_auction.find_end(amount, limit,
                                                              expiry, now)
                    if finish is None:
                        finish = inf  # None means "never finishes"
                        self.assertTrue(False, "finish is None!")
                    # Sanity
                    self.assertLessEqual(finish, expiry)
                    self.assertGreaterEqual(finish, now)

                    # Validity
                    self.assertLessEqual(
                        amount * (finish - now),
                        limit
                    )
                    self.assertLessEqual(spent, limit)

                    if finish < expiry:
                        # Didn't give up early for nothing
                        # Not possible to have spent more
                        self.assertLess(limit - spent, amount)
                    else:
                        # Spent the right amount of money
                        self.assertEqual(divmod(spent, (expiry - now)),
                                         (amount, 0))

class Test_run_auction(unittest.TestCase):
    def test_valid(self):
        pass

# Constants
class TestConstants(unittest.TestCase):
    def test_NANOSECONDS_PER_DAY(self):
        self.assertEqual(infinite_auction.NANOSECONDS_PER_DAY,
                         1000**3 * 60 * 60 * 24)

if __name__ == "__main__":
    unittest.main()
