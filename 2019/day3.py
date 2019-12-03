#!/usr/bin/env python3

import sys
import re

OFFSETS = {
    'U': (0, -1),
    'D': (0, 1),
    'L': (-1, 0),
    'R': (1, 0),
}

def moves_for_wire(wire_desc):
    for move in wire_desc.split(','):
        md = re.match(r'([UDLR])([0-9]+)', move)
        yield (md.group(1), int(md.group(2)))

def positions_for_wire(wire_desc):
    moves = list(moves_for_wire(wire_desc))

    def get_positions():
        x = 0
        y = 0
        distance = 0

        for d, c in moves:
            offset = OFFSETS[d]

            for i in range(c):
                x += offset[0]
                y += offset[1]
                distance += 1

                yield x, y, distance
            
    return get_positions()

def crossovers(a, b):
    b_positions = {}

    for x, y, d in positions_for_wire(b):
        pos = (x, y)
        try:
            old_d = b_positions[pos]
        except KeyError:
            b_positions[pos] = d
        else:
            if old_d > d:
                b_positions[pos] = d

    for x, y, d in positions_for_wire(a):
        pos = (x, y)
        if pos in b_positions:
            yield x, y, d + b_positions[pos]

def manhattan_distance(pos):
    return abs(pos[0]) + abs(pos[1])

a = next(sys.stdin)
b = next(sys.stdin)

best_crossover = min(crossovers(a, b), key=manhattan_distance)

print("Part 1: {}".format(manhattan_distance(best_crossover)))

shortest_signal = min(crossovers(a, b), key=lambda x: x[2])

print("Part 2: {}".format(shortest_signal[2]))
