#!/usr/bin/env python3

import sys
import re

START_SHIFT_RE = re.compile(r'Guard #([0-9]+) begins shift')
SLEEP_RE = re.compile(r':([0-9]+)\] falls asleep')
WAKE_UP_RE = re.compile(r':([0-9]+)\] wakes up')

current_guard = None
sleep_minute = None
guards = {}

for line in sorted(sys.stdin):
    md = START_SHIFT_RE.search(line)
    if md:
        current_guard = int(md.group(1))
        continue
    md = SLEEP_RE.search(line)
    if md:
        sleep_minute = int(md.group(1))
        continue
    md = WAKE_UP_RE.search(line)
    if md:
        wake_minute = int(md.group(1))
        try:
            sleeps = guards[current_guard]
        except KeyError:
            sleeps = []
            guards[current_guard] = sleeps
        sleeps.append((sleep_minute, wake_minute))
        continue

def count_sleep_time(sleeps):
    return sum(b - a for a, b in sleeps)

def find_sleepy_minute(sleeps):
    minutes = [0] * 60
    for start, end in sleeps:
        for m in range(start, end):
            minutes[m] += 1

    return max(enumerate(minutes), key=lambda x: x[1])
    
sleepy_guard = max(guards.items(), key=lambda x: count_sleep_time(x[1]))
sleepy_minute = find_sleepy_minute(sleepy_guard[1])[0]

print("Part 1: {}".format(sleepy_minute * sleepy_guard[0]))

consistent_guard = max(guards.items(),
                       key=lambda x: find_sleepy_minute(x[1])[1])

consistent_minute = find_sleepy_minute(consistent_guard[1])[0]

print("Part 2: {}".format(consistent_guard[0] * consistent_minute))
