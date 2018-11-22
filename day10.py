#!/usr/bin/env python3

import sys

def reverse_section(string, start, length):
    for i in range(length // 2):
        a = (start + i) % len(string)
        b = (start + length - i - 1) % len(string)
        tmp = string[a]
        string[a] = string[b]
        string[b] = tmp


skip_size = 0
current_pos = 0
string = list(range(256))
lengths = [int(x) for x in sys.stdin.read().split(',')]

for length in lengths:
    reverse_section(string, current_pos, length)
    current_pos += (skip_size + length) % len(string)
    skip_size += 1

print("Part 1: {}".format(string[0] * string[1]))
