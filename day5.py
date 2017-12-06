#!/usr/bin/python3

import sys

jumps = [int(line) for line in sys.stdin]

pc = 0
steps = 0

while pc >= 0 and pc < len(jumps):
    offset = jumps[pc]
    jumps[pc] += 1
    pc += offset
    steps += 1

print(steps)
