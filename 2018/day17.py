#!/usr/bin/env python3

import sys
import re
import itertools

CLAY_RE = re.compile(r'([xy])=([0-9]+), ([xy])=([0-9]+)\.\.([0-9]+)$')

class Terrain:
    def __init__(self, lines):
        self.clay = set()

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

        self.max_y = max(coord[1] for coord in self.clay)
        self.visited = dict()

    def is_clay(self, x, y):
        return (x, y) in self.clay

    def count_water(self, x, y):
        ret = self.count_water_i(x, y)
        print("{},{}: {}".format(x, y, ret))
        return ret

    def count_water_i(self, x, y):
        try:
            return (0, self.visited[(x, y)])
        except KeyError:
            pass

        if y > self.max_y:
            return (0, False)

        if self.is_clay(x, y):
            return (0, True)

        clay_count, below_settle = self.count_water(x, y + 1)

        clay_count += 1

        self.visited[(x, y)] = below_settle

        if not below_settle:
            return (clay_count, False)

        can_settle = True

        for iterator in (itertools.count(x - 1, -1),
                         itertools.count(x + 1, +1)):
            for xx in iterator:
                if self.is_clay(xx, y):
                    break

                (below_count, below_settle) = self.count_water(xx, y + 1)

                clay_count += 1 + below_count

                if not below_settle:
                    can_settle = False
                    break

        return (clay_count, can_settle)


terrain = Terrain(sys.stdin)

print("Part 1: {}".format(terrain.count_water(500, 0)[0]))
