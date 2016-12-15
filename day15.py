import sys
import re

class DiscCondition:
    def __init__(self, mod, offset):
        self._mod = mod
        self._offset = offset

    def is_good(self, time):
        return (self._offset + time) % self._mod == 0

def read_discs():
    disc_re = re.compile(r'Disc #([0-9]+) has ([0-9]+) positions; '
                         r'at time=0, it is at position ([0-9]+).')
    for line in sys.stdin:
        md = disc_re.match(line)
        yield DiscCondition(int(md.group(2)),
                            int(md.group(1)) + int(md.group(3)))

def check_time(discs, start_time):
    for disc in discs:
        if not disc.is_good(start_time):
            return False

    return True

def solve(discs):
    start_time = 0

    while True:
        if check_time(discs, start_time):
            return start_time

        start_time += 1

discs = list(read_discs())

print("Part 1:", solve(discs))

discs.append(DiscCondition(11, len(discs) + 1))

print("Part 2:", solve(discs))
