#!/usr/bin/python3

import sys
import re
import struct
from collections import namedtuple


class Bot(namedtuple('Bot', 'x,y,z,radius')):
    def distance(self, other):
        return self.distance_point(other.x, other.y, other.z)

    def distance_point(self, x, y, z):
        return (abs(self.x - x) +
                abs(self.y - y) +
                abs(self.z - z))

    def coord(self, axis):
        if axis == 0:
            return self.x
        if axis == 1:
            return self.y
        if axis == 2:
            return self.z
        assert(false)

    def contains_point(self, x, y, z):
        return self.distance_point(x, y, z) <= self.radius


BOT_RE = re.compile(r'pos=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, '
                    r'r=([0-9]+)$')


def bots(input):
    for line in input:
        md = BOT_RE.match(line)
        yield(Bot(*(int(x) for x in md.groups())))

def points(bots):
    for bot in bots:
        for off in (-bot.radius, bot.radius):
            yield bot.x + off, bot.y, bot.z
            yield bot.x, bot.y + off, bot.z
            yield bot.x, bot.y, bot.z + off

def in_all(x, y, z, bots):
    for bot in bots:
        if not bot.contains_point(x, y, z):
            return False
    return True

def max_move(point, axis, bots):
    offset = sys.maxsize
    for bot in bots:
        max_axis_move = bot.radius - (abs(point[(axis + 1) % 3] -
                                          bot.coord((axis + 1) % 3)) +
                                      abs(point[(axis + 2) % 3] -
                                          bot.coord((axis + 2) % 3)))
        if point[axis] < 0:
            bot_offset = max_axis_move - (point[axis] - bot.coord(axis))
        else:
            bot_offset = max_axis_move - (bot.coord(axis) - point[axis])

        if bot_offset < offset:
            offset = bot_offset

    return offset

all_bots = list(bots(sys.stdin))

strongest_bot = max(all_bots, key=lambda bot: bot.radius)

in_range = sum(1 for bot in all_bots
               if bot.distance(strongest_bot) <= strongest_bot.radius)

print("Part 1: {}".format(in_range))

best_point = (0, 0, 0)
best_count = 0
best_bot = None

for point in points(all_bots):
    in_bots = sum(1 for bot in all_bots if bot.contains_point(*point))

    if best_count <= in_bots:
        best_point = point
        best_count = in_bots
        print(best_point, best_count)

dist = sum(abs(v) for v in best_point)

print("{} ({}: {} {} {})".format(dist, best_count, *best_point))

touching_bots = [bot for bot in all_bots if bot.contains_point(*best_point)]
point = best_point

while True:
    for i in range(3):
        offset = max_move(point, i, touching_bots)
        if offset > 0:
            if point[i] > 0:
                new_value = point[i] - offset
            else:
                new_value = point[i] + offset
            if i == 0:
                point = (new_value, point[1], point[2])
            elif i == 1:
                point = (point[0], new_value, point[2])
            elif i == 2:
                point = (point[0], point[1], new_value)
            break
    else:
        break

dist = sum(abs(v) for v in point)
print("Part 2: {} ({} {} {})".format(dist, *point))

assert(in_all(*point, touching_bots))
