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
    def test_alice_bids(self):
        # Alice starts out and bids a maximum of $5 a day for a week.
        # Since she's the only bidder, her bid starts out at $0.
        # Free advertising!
        auction = infinite_auction.run_auction(
            (infinite_auction.Bid(
                500,  # $5
                42,   # unspecified (should default to 500 * 7 * N_PER_DAY)
                7 * infinite_auction.NANOSECONDS_PER_DAY,
                "Alice"
            ),),
            10,       # 10¢
            0         # t=0
        )
        alice, expiry, spent = next(auction)
        self.assertEqual(alice.id, "Alice")
        self.assertEqual(expiry, 7 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(spent, 0)

    def test_partario_outbid(self):
        # Partario wants in! He sees Alice's bid of $0 and decides
        # he'll outbid her with a bid of $1 a day for one day.
        # He places his bid, but Alice's bid automatically outbids him.
        auction = infinite_auction.run_auction(
            (
                infinite_auction.Bid(
                    500,  # $5
                    110 * infinite_auction.NANOSECONDS_PER_DAY + 42,
                    7 * infinite_auction.NANOSECONDS_PER_DAY,
                    "Alice"
                ),
                infinite_auction.Bid(
                    100,  # $1
                    42,   # anything with 42 means unspecified
                    1 * infinite_auction.NANOSECONDS_PER_DAY,
                    "Partario"
                )
            ),
            10,       # 10¢
            0         # t=0
        )
        alice, expiry, spent = next(auction)
        self.assertEqual(alice.id, "Alice")
        self.assertEqual(expiry, 1 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(spent, 110 * infinite_auction.NANOSECONDS_PER_DAY)

        # A day later, Partario's bid expires, and Alice's bid
        # automatically drops back down to $0.
        # Free advertising!
        alice, expiry, spent = next(auction)
        self.assertEqual(alice.id, "Alice")
        self.assertEqual(expiry, 7 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(spent, 0)

        with self.assertRaises(StopIteration):
            next(auction)

    def test_partarios_revenge(self):
        # Partario's back, and this time he decides to bid $100 a day
        # for a week, with an expense limit of $1. Alice is outbid!
        # Notice that Partario's bid only goes as high as it needs to
        # in order to defeat Alice.
        auction = infinite_auction.run_auction(
            (
                infinite_auction.Bid(
                    500,                                         # $5
                    42,
                    7 * infinite_auction.NANOSECONDS_PER_DAY,
                    "Alice"
                ),
                infinite_auction.Bid(
                    10000,                                       # $100
                    100 * infinite_auction.NANOSECONDS_PER_DAY,  # $1
                    1 * infinite_auction.NANOSECONDS_PER_DAY,
                    "Partario"
                )
            ),
            10,       # 10¢
            0         # t=0
        )

        # Since Partario's bid is pretty big, but his expense limit is pretty
        # small, the $1 expense limit is very quickly reached. His bid was
        # chargd at $5.10/day to a maximum of $1, so he got about five hours of
        # display time ($5.10/day is about 20 cents an hour) before
        # that $1 limit was reached.
        partario, expiry, spent = next(auction)
        self.assertEqual(partario.id, "Partario")
        self.assertAlmostEqual(
            expiry / infinite_auction.NANOSECONDS_PER_DAY * 24,
            5, places=0)
        self.assertAlmostEqual(
            spent / infinite_auction.NANOSECONDS_PER_DAY,
            100       # $1
        )

        # His bid expires, and Alice's takes over again at $0.
        # Free advertising!
        alice, expiry, spent = next(auction)
        self.assertEqual(alice.id, "Alice")
        self.assertEqual(expiry, 7 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(spent, 0)

        with self.assertRaises(StopIteration):
            next(auction)

    def test_partinos_revenge_2(self):
        # Partino also decides to place a second bid, this time of $1 a day
        # for a week. This way, if his more expensive bid expires, he'll still
        # have a chance of his ad being shown at a rate he likes! This new bid
        # is outbid, but his older bid of $100 (max) is still the high bidder,
        # so his ad is being shown.
        auction = infinite_auction.run_auction(
            (
                infinite_auction.Bid(
                    500,                                         # $5
                    500 * 7 * infinite_auction.NANOSECONDS_PER_DAY + 42,
                    7 * infinite_auction.NANOSECONDS_PER_DAY,
                    "Alice"
                ),
                infinite_auction.Bid(
                    10000,                                       # $100
                    100 * infinite_auction.NANOSECONDS_PER_DAY,  # $1
                    8 * infinite_auction.NANOSECONDS_PER_DAY,  # started at 1d
                    "Partario"
                ),
                infinite_auction.Bid(
                    100,                # $1
                    42,
                    8 * infinite_auction.NANOSECONDS_PER_DAY,  # started at 1d
                    "Partario2"
                )
            ),
            10,                                   # 10¢
            infinite_auction.NANOSECONDS_PER_DAY  # t=1d
        )
        partario, expiry, spent = next(auction)
        self.assertEqual(partario.id, "Partario")
        self.assertAlmostEqual(
            expiry / infinite_auction.NANOSECONDS_PER_DAY * 24,
            24 + 5, places=0)
        self.assertAlmostEqual(
            spent / infinite_auction.NANOSECONDS_PER_DAY,
            100       # $1
        )

        # Since Partario's bid is pretty big, but his expense limit is pretty
        # small, the $1 expense limit is very quickly reached. His bid expires,
        # and once again Alice is the highest bidder. Partario's second bid of
        # $1 a day forces Alice's bid up a bit.
        alice, expiry_2, spent_2 = next(auction)
        self.assertEqual(alice.id, "Alice")
        self.assertEqual(expiry_2, 7 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(spent_2, (expiry_2 - expiry) * 110)

        # Finally, a week after she placed it, Alice's bid expiry date is
        # reached, and it expires.
        # Despite Project Wonderful's simplified example, there's still a bit
        # left in Partario's first bid. He's bidding against himself!
        partario, expiry_2, spent_2 = next(auction)
        self.assertEqual(partario.id, "Partario")
        ## self.assertEqual(expiry_2,  # don't know what this should be!
        self.assertGreaterEqual(spent + spent_2, partario.expense_limit - 110)

        # Partario's bid of $1 a day is now the high bidder!
        # Nobody else has bid, so: free advertising!
        partario, expiry, spent = next(auction)
        self.assertEqual(partario.id, "Partario2")
        self.assertEqual(expiry, 8 * infinite_auction.NANOSECONDS_PER_DAY)
        self.assertEqual(spent, 0)

        with self.assertRaises(StopIteration):
            next(auction)

    def test_none_can_bid(self):
        auction = infinite_auction.run_auction(
            (
                infinite_auction.Bid(
                    500,
                    400,
                    10000,
                    "Can't pay."
                ),
                infinite_auction.Bid(
                    500,
                    600,
                    10000,
                    "Can only pay once."
                ),
                infinite_auction.Bid(
                    450,
                    -1,
                    10000,
                    "Drives the price up."
                )
            ),
            10,
            0
        )
        bid, expiry, spent = next(auction)
        self.assertEqual(bid.id, "Can only pay once.")
        self.assertEqual(expiry, 1)
        self.assertEqual(spent, 460)

        with self.assertRaises(StopIteration):
            next(auction)

# Constants
class TestConstants(unittest.TestCase):
    def test_NANOSECONDS_PER_DAY(self):
        self.assertEqual(infinite_auction.NANOSECONDS_PER_DAY,
                         1000**3 * 60 * 60 * 24)

if __name__ == "__main__":
    unittest.main()
