#!/usr/bin/python3

import sys
import re

MASK_RE = re.compile(r'mask = ([X01]{36})$')
MEM_RE = re.compile(r'mem\[([0-9]+)\] = ([0-9]+)$')

def count_bits(value):
    bits = 0
    high_bit = 0

    while value > 0:
        if value & 1:
            bits += 1
        value >>= 1
        high_bit += 1

    return bits, high_bit - 1

class Mask:
    def __init__(self, mask):
        self.clear_mask = 0
        self.set_mask = 0
        self.float_mask = 0

        for ch in mask:
            self.clear_mask <<= 1
            self.set_mask <<= 1
            self.float_mask <<= 1

            if ch == '0':
                self.clear_mask |= 1
            elif ch == '1':
                self.set_mask |= 1
            elif ch == 'X':
                self.float_mask |= 1

        self.n_float_bits, self.high_bit = count_bits(self.float_mask)

    def apply_part1(self, value):
        return (value & ~self.clear_mask) | self.set_mask

    def _floating_values(self):
        for i in range(1 << self.n_float_bits):
            fm = self.float_mask
            v = 0

            while fm != 0:
                v >>= 1
                if (fm & 1) != 0:
                    if (i & 1) != 0:
                        v |= (1 << self.high_bit)
                    i >>= 1
                fm >>= 1

            yield v            

    def addresses_part2(self, base_addr):
        for addr in self._floating_values():
            yield (base_addr & self.clear_mask) | self.set_mask | addr

mem_part1 = {}
mem_part2 = {}

for line in sys.stdin:
    md = MASK_RE.match(line)
    if md:
        mask = Mask(md.group(1))
        continue

    md = MEM_RE.match(line)
    if md:
        base_addr = int(md.group(1))
        value = int(md.group(2))

        mem_part1[base_addr] = mask.apply_part1(value)

        for addr in mask.addresses_part2(base_addr):
            mem_part2[addr] = value

        continue

    raise "Bad line: {}".format(line.strip())

print("Part 1: {}".format(sum(mem_part1.values())))
print("Part 2: {}".format(sum(mem_part2.values())))
