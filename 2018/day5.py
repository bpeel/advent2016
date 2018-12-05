#!/usr/bin/env python3

import sys

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

polymer = list(sys.stdin.read().rstrip())

while step(polymer):
    pass

print("Part 1: {}".format(len(polymer)))
