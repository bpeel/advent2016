#!/usr/bin/env python3

import sys
import re

POINT_RE = re.compile(r'([0-9]+), ([0-9]+)$')

class Data:
    def __init__(self, points):
        self.points = list(points)
        self.min_x = sys.maxsize
        self.max_x = -sys.maxsize
        self.min_y = sys.maxsize
        self.max_y = -sys.maxsize

        for point in self.points:
            self.min_x = min(self.min_x, point[0])
            self.max_x = max(self.max_x, point[0])
            self.min_y = min(self.min_y, point[1])
            self.max_y = max(self.max_y, point[1])

def parse_point(line):
    md = POINT_RE.match(line)
    return (int(md.group(1)), int(md.group(2)))

def point_distance(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

def point_area_size(data, point):
    area_points = set()
    stack = [point]

    while len(stack) > 0:
        try_point = stack.pop()
        if try_point in area_points:
            continue

        closest_a = None
        closest_b = None
        closest_a_point = None

        for p in data.points:
            dist = point_distance(p, try_point)
            if closest_a is None or dist < closest_a:
                closest_b = closest_a
                closest_a = dist
                closest_a_point = p
            elif closest_b is None or dist < closest_b:
                closest_b = dist

        # If a different point is closer or equally close then stop searching
        if closest_a == closest_b or closest_a_point is not point:
            continue

        # If the point is outside the range then the area is infinite
        # so we’ll just ignore it
        if (try_point[0] < data.min_x or try_point[0] > data.max_x or
            try_point[1] < data.min_y or try_point[1] > data.max_y):
            return -sys.maxsize

        area_points.add(try_point)

        stack.append((try_point[0], try_point[1] - 1))
        stack.append((try_point[0], try_point[1] + 1))
        stack.append((try_point[0] - 1, try_point[1]))
        stack.append((try_point[0] + 1, try_point[1]))

    return len(area_points)

data = Data(parse_point(line) for line in sys.stdin)

part1 = max(point_area_size(data, point) for point in data.points)

print("Part 1: {}".format(part1))
