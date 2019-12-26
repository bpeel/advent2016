#!/usr/bin/env python3

import sys
import subprocess

def parse_map(infile):
    return [line.rstrip() for line in infile if len(line) >= 2]

def get_map():
    if len(sys.argv) > 1:
        proc = subprocess.Popen(["./build/day25", sys.argv[1]],
                                stdout=subprocess.PIPE,
                                text=True,
                                encoding="utf-8")
        return parse_map(proc.stdout)
    else:
        return parse_map(sys.stdin)

def get_intersections(scaf_map):
    for y in range(1, len(scaf_map) - 1):
        for x in range(1, len(scaf_map[y]) - 1):
            if scaf_map[y][x] != '#':
                continue

            for i in range(-1, 2, 2):
                if (scaf_map[y + i][x] != '#' or
                    scaf_map[y][x + i] != '#'):
                    break
            else:
                yield (x, y)

print("Part 1: ", sum(a * b for a, b in get_intersections(get_map())))
