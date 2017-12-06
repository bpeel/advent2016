#!/usr/bin/python3

import sys

def execute(jumps, decrease = False):
    pc = 0
    steps = 0

    while pc >= 0 and pc < len(jumps):
        offset = jumps[pc]
        if decrease and offset >= 3:
            jumps[pc] -= 1
        else:
            jumps[pc] += 1
        pc += offset
        steps += 1

    return steps

jumps = [int(line) for line in sys.stdin]

print("Part 1", execute(list(jumps)))
print("Part 2", execute(list(jumps), True))
