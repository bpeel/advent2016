#!/usr/bin/env python3

import sys

def reverse_section(string, start, length):
    for i in range(length // 2):
        a = (start + i) % len(string)
        b = (start + length - i - 1) % len(string)
        tmp = string[a]
        string[a] = string[b]
        string[b] = tmp


class Hasher:
    def __init__(self):
        self.skip_size = 0
        self.current_pos = 0
        self.string = list(range(256))

    def process(self, lengths):
        for length in lengths:
            reverse_section(self.string, self.current_pos, length)
            self.current_pos += (self.skip_size + length) % len(self.string)
            self.skip_size += 1


input_string = sys.stdin.read().strip()

hasher = Hasher()
hasher.process(int(x) for x in input_string.split(','))

print("Part 1: {}".format(hasher.string[0] * hasher.string[1]))

hasher = Hasher()
lengths = [ord(c) for c in input_string] + [17, 31, 73, 47, 23]

for i in range(64):
    hasher.process(lengths)

def hash_all(funcs):
    i = iter(funcs)
    result = next(i)

    for v in i:
        result ^= v

    return result

hash = [hash_all(hasher.string[(i * 16):((i + 1) * 16)]) for i in range(16)]
hash_str = "".join("{:02x}".format(x) for x in hash)
print("Part 2: {}".format(hash_str))
