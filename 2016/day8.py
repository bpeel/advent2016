import sys
import re
from itertools import permutations

WIDTH = 50
HEIGHT = 6

pixels = [0] * (WIDTH * HEIGHT)

def process_line(pixels, line):
    md = re.match(r'rect ([0-9]+)x([0-9]+)$', line)
    if md:
        w = int(md.group(1))
        h = int(md.group(2))
        for y in range(h):
            for x in range(w):
                pixels[x + y * WIDTH] = 1
        return
    md = re.match(r'rotate row y=([0-9]+) by ([0-9]+)$', line)
    if md:
        y = int(md.group(1))
        by = int(md.group(2))
        if by == 0:
            return
        left = list(pixels[y * WIDTH : (WIDTH - by) + y * WIDTH])
        right = list(pixels[(WIDTH - by) + y * WIDTH : WIDTH + y * WIDTH])
        pixels[y * WIDTH : by + y * WIDTH] = right
        pixels[by + y * WIDTH : WIDTH + y * WIDTH] = left
        return
    md = re.match(r'rotate column x=([0-9]+) by ([0-9]+)$', line)
    if md:
        x = int(md.group(1))
        by = int(md.group(2))
        if by == 0:
            return
        top = [pixels[x + y * WIDTH] for y in range(HEIGHT - by)]
        bottom = [pixels[x + (HEIGHT - by + y) * WIDTH] for y in range(by)]
        for y, v in enumerate(bottom):
            pixels[x + y * WIDTH] = v
        for y, v in enumerate(top):
            pixels[x + (by + y) * WIDTH] = v
        return
    raise ValueError("Bad line " + line.rstrip())

for line in sys.stdin:
    process_line(pixels, line)

print("Part 1", sum(pixels))

print("Part 2")
for y in range(HEIGHT):
    for x in range(WIDTH):
        print([' ', '#'][pixels[x + y * WIDTH]], end='')
    print()
