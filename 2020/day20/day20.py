#!/usr/bin/python3

import sys
import re

# AB  BA  CD  BA
# CD  DC  AB  DC

# CA  AC  DB  BD
# DB  BD  CA  AC

# Each pattern has 8 patterns
# Rotate 90° or not, flip X or not, flip Y or not
# 2³=8

class Tile:
    def _parse_line(line):
        return int(line.replace('.', '0').replace('#', '1'), 2)

    def __init__(self, id, lines):
        self.id = id
        self.edges = [
            # Top
            Tile._parse_line(lines[0]),
            # Right
            Tile._parse_line(''.join(x[-1] for x in lines)),
            # Bottom
            Tile._parse_line(lines[-1]),
            # Left
            Tile._parse_line(''.join(x[0] for x in lines))
        ]

def get_tiles(lines):
    num = None
    grid = []

    for line in lines:
        line = line.rstrip()

        if num is None:
            num = int(re.match(r'Tile ([0-9]+):$', line).group(1))
        elif len(line) == 0:
            yield Tile(num, grid)
            num = None
            grid.clear()
        else:
            grid.append(line)
        
for x in next(get_tiles(sys.stdin)).edges:
    print(bin(x))
