#!/usr/bin/env python3

import sys
import re
import itertools

CLAY_RE = re.compile(r'([xy])=([0-9]+), ([xy])=([0-9]+)\.\.([0-9]+)$')

class Terrain:
    def __init__(self, lines):
        self.clay = set()
        self.can_settle_cache = dict()

        for line in lines:
            md = CLAY_RE.match(line)
            (fix_axis, fix_coord, range_axis, range_start, range_end) = \
                md.groups()
            assert(fix_axis != range_axis)
            fix_coord = int(fix_coord)
            range_start = int(range_start)
            range_end = int(range_end)
            assert(range_end >= range_start)

            for i in range(range_start, range_end + 1):
                if fix_axis == 'x':
                    coord = (fix_coord, i)
                else:
                    coord = (i, fix_coord)

                self.clay.add(coord)

        self.min_y = min(coord[1] for coord in self.clay)
        self.max_y = max(coord[1] for coord in self.clay)

    def is_clay(self, x, y):
        return (x, y) in self.clay

    def can_settle(self, x, y):
        try:
            return self.can_settle_cache[(x, y)]
        except KeyError:
            ret = self._can_settle(x, y)
            self.can_settle_cache[(x, y)] = ret
            return ret

    def _can_settle(self, x, y):
        if y > self.max_y:
            return False

        if self.is_clay(x, y):
            return True

        if not self.can_settle(x, y + 1):
            return False

        for step in (-1, 1):
            for xx in itertools.count(x + step, step):
                if self.is_clay(xx, y):
                    break
                if not self.can_settle(xx, y + 1):
                    return False

        return True

class Counter:
    def __init__(self, terrain):
        self.terrain = terrain
        self.water = set()
        self.forever_water = set()

    def add_water(self, x, y):
        if y > self.terrain.max_y:
            return

        if self.terrain.is_clay(x, y):
            return

        if (x, y) in self.water:
            return

        self.water.add((x, y))

        self.add_water(x, y + 1)

        if self.terrain.can_settle(x, y + 1):
            is_forever = True

            for step in (-1, 1):
                for xx in itertools.count(x + step, step):
                    if self.terrain.is_clay(xx, y):
                        break

                    self.water.add((xx, y))
                    self.add_water(xx, y + 1)

                    if not self.terrain.can_settle(xx, y + 1):
                        is_forever = False
                        break

            if is_forever:
                self.forever_water.add((x, y))
                for step in (-1, 1):
                    for xx in itertools.count(x + step, step):
                        if self.terrain.is_clay(xx, y):
                            break
                        self.forever_water.add((xx, y))

sys.setrecursionlimit(10000)

if len(sys.argv) > 1:
    with open(sys.argv[1], 'r') as f:
        terrain = Terrain(f)
else:
    terrain = Terrain(sys.stdin)
counter = Counter(terrain)
counter.add_water(500, terrain.min_y)

print("Part 1: {}".format(len(counter.water)))
print("Part 2: {}".format(len(counter.forever_water)))
