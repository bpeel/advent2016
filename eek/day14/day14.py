#!/usr/bin/python3

import sys
import re

MASK_RE = re.compile(r'mask = ([X01]{36})$')
MEM_RE = re.compile(r'mem\[([0-9]+)\] = ([0-9]+)$')

def parse_mask(mask):
    clear_mask = 0
    set_mask = 0

    for ch in mask:
        clear_mask <<= 1
        set_mask <<= 1

        if ch == '0':
            clear_mask |= 1
        elif ch == '1':
            set_mask |= 1

    return clear_mask, set_mask

def apply_mask(clear_mask, set_mask, value):
    return (value & ~clear_mask) | set_mask

mem = {}

for line in sys.stdin:
    md = MASK_RE.match(line)
    if md:
        clear_mask, set_mask = parse_mask(md.group(1))
        continue

    md = MEM_RE.match(line)
    if md:
        mem[int(md.group(1))] = apply_mask(clear_mask, set_mask, int(md.group(2)))
        continue

    raise "Bad line: {}".format(line.strip())

print(mem)

print(sum(mem.values()))
