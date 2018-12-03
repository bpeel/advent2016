#!/usr/bin/env python3

import sys
from collections import namedtuple
import re

class Rect(namedtuple('Rect', 'id, left, top, width, height')):
    @property
    def x1(self):
        return self.left
    @property
    def x2(self):
        return self.left + self.width
    @property
    def y1(self):
        return self.top
    @property
    def y2(self):
        return self.top + self.height

    def intersects(rect_a, rect_b):
        return (rect_a.x2 > rect_b.x1 and rect_a.x1 < rect_b.x2 and
                rect_a.y2 > rect_b.y1 and rect_a.y1 < rect_b.y2)

    def intersection(rect_a, rect_b):
        if not rect_a.intersects(rect_b):
            return None

        x1 = max(rect_a.x1, rect_b.x1)
        x2 = min(rect_a.x2, rect_b.x2)
        y1 = max(rect_a.y1, rect_b.y1)
        y2 = min(rect_a.y2, rect_b.y2)

        return Rect(rect_a.id, x1, y1, x2 - x1, y2 - y1)

    def subtract(rect_a, rect_b):
        inter = rect_a.intersection(rect_b)

        if inter is None:
            yield rect_a
            return

        if inter.x1 > rect_a.x1:
            yield Rect(rect_a.id,
                       rect_a.x1,
                       rect_a.top,
                       inter.x1 - rect_a.x1,
                       rect_a.height)
        if rect_a.x2 > inter.x2:
            yield Rect(rect_a.id,
                       inter.x2,
                       rect_a.top,
                       rect_a.x2 - inter.x2,
                       rect_a.height)
        if inter.y1 > rect_a.y1:
            yield Rect(rect_a.id,
                       inter.x1,
                       rect_a.y1,
                       inter.x2 - inter.x1,
                       inter.y1 - rect_a.y1)
        if rect_a.y2 > inter.y2:
            yield Rect(rect_a.id,
                       inter.x1,
                       inter.y2,
                       inter.x2 - inter.x1,
                       rect_a.y2 - inter.y2)

class Region:
    def __init__(self):
        self.rects = []

    def add(self, rect):
        to_add = [rect]

        for r_rect in self.rects:
            next_to_add = []
            for add_rect in to_add:
                next_to_add.extend(add_rect.subtract(r_rect))
            to_add = next_to_add

        self.rects.extend(to_add)

    def intersection(self, rect):
        for r_rect in self.rects:
            inter = r_rect.intersection(rect)
            if inter is not None:
                yield inter

    def area(self):
        return sum(rect.width * rect.height for rect in self.rects)

RECT_RE = re.compile(r'#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)')

def parse_rect(line):
    md = RECT_RE.match(line)
    return Rect(*(int(x) for x in md.groups()))

rects = [parse_rect(line) for line in sys.stdin]

claimed_region = Region()
overlap_region = Region()

for rect in rects:
    for inter in claimed_region.intersection(rect):
        overlap_region.add(inter)
    claimed_region.add(rect)

print("Part 1: {}".format(overlap_region.area()))
