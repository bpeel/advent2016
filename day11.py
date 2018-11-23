#!/usr/bin/env python3

import sys

# Let’s just treat this as a square grid but in addition to being able
# to move up/down/left/right you can also move up+right and down+left.
# That way if the final position is above and to the right you can
# take a shortcut by moving some of the up movements as up+right
# movements as well.

def hex_distance(pos):
    if (pos[0] > 0) == (pos[1] > 0):
        diagonal_movement = min(abs(pos[0]), abs(pos[1]))
        if pos[0] < 0:
            diagonal_movement = -diagonal_movement
        pos = (pos[0] - diagonal_movement, pos[1] - diagonal_movement)
    else:
        diagonal_movement = 0

    return sum(abs(x) for x in pos) + diagonal_movement


movements = {
    'n': (0, 1),
    'ne': (1, 1),
    'se': (1, 0),
    's': (0, -1),
    'sw': (-1, -1),
    'nw': (-1, 0)
}

pos = (0, 0)
max_distance = 0

for direction in sys.stdin.read().rstrip().split(','):
    diff = movements[direction]
    pos = (pos[0] + diff[0], pos[1] + diff[1])
    max_distance = max(max_distance, hex_distance(pos))

print("Final position {}".format(pos))

print(("Part 1: {}\n"
       "Part 2: {}").format(hex_distance(pos), max_distance))
