#!/usr/bin/env python3

import sys
import re

def step(polymer):
    outpos = 0
    inpos = 0
    limit = len(polymer) - 1

    while inpos < limit:
        a = polymer[inpos]
        b = polymer[inpos + 1]
        if a.isupper() == b.isupper() or a.lower() != b.lower():
            polymer[outpos] = a
            outpos += 1
            inpos += 1
        else:
            inpos += 2

    if inpos < len(polymer):
        polymer[outpos] = polymer[-1]
        outpos += 1

    modified = outpos != len(polymer)

    del polymer[outpos:]

    return modified

def reduce_polymer(input):
    polymer = list(input)
    while step(polymer):
        pass
    return len(polymer)

def strip_reduce_polymer(input, ch):
    return reduce_polymer(re.sub(ch, "", input, flags=re.IGNORECASE))

input = sys.stdin.read().rstrip()

print("Part 1: {}".format(reduce_polymer(input)))

part2 = min(((chr(ch), strip_reduce_polymer(input, chr(ch)))
             for ch in range(ord("a"), ord("z") + 1)),
            key=lambda x: x[1])

print("Part 2: {} {}".format(*part2))
