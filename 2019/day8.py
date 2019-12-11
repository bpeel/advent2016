#!/usr/bin/env python3

import sys

WIDTH = 25
HEIGHT = 6

buf = [int(x) for x in sys.stdin.read().strip()]

assert(len(buf) % (WIDTH * HEIGHT) == 0)

layers = [buf[x * WIDTH * HEIGHT : (x + 1) * WIDTH * HEIGHT]
          for x in range(len(buf) // (WIDTH * HEIGHT))]

def count_digits(layer, digit):
    return sum(1 for x in layer if x == digit)

best_layer = min(layers, key=lambda x: count_digits(x, 0))

print("Part 1: {}".format(count_digits(best_layer, 1) *
                          count_digits(best_layer, 2)))
