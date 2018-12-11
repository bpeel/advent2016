#!/usr/bin/env python3

import sys

GRID_SIZE = 300

def power_level(x, y, serial_number):
    rack_id = x + 11
    base_power_level = rack_id * (y + 1)
    increase_power_level = base_power_level + serial_number
    rack_power_level = increase_power_level * rack_id
    hundreds = rack_power_level // 100 % 10
    return hundreds - 5

def square_power_level(x, y, serial_number):
    return sum(power_level(x + xx, y + yy, serial_number)
               for xx in range(3) for yy in range(3))

if len(sys.argv) != 2:
    print("usage: {} <serial_number>".format(sys.argv[0]), file=sys.stderr)
    sys.exit(1)

serial_number = int(sys.argv[1])

best_square = max(((x, y)
                   for x in range(GRID_SIZE - 2)
                   for y in range(GRID_SIZE - 2)),
                  key = lambda a: square_power_level(*a, serial_number))

print("Part 1: {},{}".format(best_square[0] + 1, best_square[1] + 1))
