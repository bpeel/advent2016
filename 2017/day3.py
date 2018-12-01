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

def arc_for_address(address):
    arc_frac = (math.sqrt(16 * address) - 4) / 8
    return math.ceil(arc_frac)

def addresses_for_size(arc):
    return 4 * arc ** 2 + 4 * arc + 1

def addresses_in_arc(arc):
    return arc * 8

def part1(address):
    arc = arc_for_address(address)
    addresses_in_this_arc = addresses_in_arc(arc)
    before_arc = addresses_for_size(arc - 1)
    position_in_arc = address - before_arc - 1
    side_length = arc * 2
    in_side = position_in_arc % side_length

    return abs(in_side - arc + 1) + arc

def address_for_pos(x, y):
    arc = max(abs(x), abs(y))
    address = addresses_for_size(arc - 1)
    addresses_in_this_arc = addresses_in_arc(arc)
    
    if x == arc and y > -arc:
        address += y + arc
    elif y == arc:
        address += arc - x + addresses_in_this_arc // 4
    elif x == -arc:
        address += arc - y + addresses_in_this_arc * 2 // 4
    else:
        address += x + arc + addresses_in_this_arc * 3 // 4

    return address
        
def part2(goal):
    memory = [ 1 ]
    while memory[-1] <= goal:
        address = len(memory) + 1
        arc = arc_for_address(address)
        addresses_in_this_arc = addresses_in_arc(arc)
        before_arc = addresses_for_size(arc - 1)
        position_in_arc = address - before_arc - 1
        side_length = arc * 2
        in_side = position_in_arc % side_length
        side_num = position_in_arc * 4 // addresses_in_this_arc

        if side_num & 1 == 0:
            x = arc
            y = 1 - arc + in_side
        else:
            x = arc - 1 - in_side
            y = arc

        if side_num & 2 == 2:
            x = -x
            y = -y

        total = 0

        for ox, oy in [(-1, 0), (1, 0), (0, -1), (0, 1),
                       (-1, -1), (1, -1), (-1, 1), (1, 1)]:
            nx = x + ox
            ny = y + oy
            naddress = address_for_pos(nx, ny)
            if naddress < address:
                total += memory[naddress - 1]

        memory.append(total)

    return memory[-1]

for arg in sys.argv[1:]:
    s = int(arg)
    print(("Part 1: {}\n"
           "Part 2: {}").format(part1(s), part2(s)))
