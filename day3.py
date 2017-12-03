#!/usr/bin/python3

# Each arc of the spiral adds 4 x 2n new addresses where n is the
# number of the spiral. This effectively forms a triangle number
# except that each row of the triangle adds 2 new addresses and there
# are four triangles to complete the spiral. Therefore, the number of
# addresses after completing spiral numer n is:
#
# s = 4n² + 4n + 1
#
# Solving this with the quadratic formula we get:
#
# n = (sqrt(16s) - 4) / 8
#
# If we round up n, then n - 1 is the number of moves required to get
# inline with the center address. Next we just need to workout how far
# to move in the other direction. We know that this arc has 8(n-1)
# addresses so we can use this to work out which quadrant we are in.
# Then we can just work out the distance to the center of this
# quadrant with a simple subtraction.

import math
import sys

def get_distance(address):
    arc_frac = (math.sqrt(16 * address) - 4) / 8
    arc = math.ceil(arc_frac)
    addresses_in_this_arc = 8 * arc
    before_arc = 4 * (arc - 1) ** 2 + 4 * (arc - 1) + 1
    position_in_arc = address - before_arc - 1
    side_length = arc * 2
    in_side = position_in_arc % side_length

    return abs(in_side - arc + 1) + arc

for arg in sys.argv[1:]:
    s = int(arg)
    print(s, get_distance(s))
