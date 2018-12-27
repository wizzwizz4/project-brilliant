import unittest
try:
    from . import infinite_auction
except (SystemError, ImportError):
    import __init__ as infinite_auction

# API

# Internals
class Test_highest_and_runner_up(unittest.TestCase):
    def test_first_come(self):
        # TODO: Add underscores back to the prices (500 -> 5_00)
        highest, runner_up_bid = infinite_auction._highest_and_runner_up((
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
        ))
        self.assertEqual(highest.id, "Winner")
        self.assertEqual(runner_up_bid,
                                  500 * infinite_auction.NANOSECONDS_PER_DAY)

# Constants
class TestConstants(unittest.TestCase):
    def test_NANOSECONDS_PER_DAY(self):
        self.assertEqual(infinite_auction.NANOSECONDS_PER_DAY,
                         1000**3 * 60 * 60 * 24)

if __name__ == "__main__":
    unittest.main()
