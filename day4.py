#!/usr/bin/python3

import sys

def password_is_valid(password, normalise):
    words = {}
    for word in password.split():
        if normalise:
            word = "".join(sorted(word))
        if word in words:
            return False
        words[word] = True

    return True

count_part1 = 0
count_part2 = 0

for line in sys.stdin:
    if password_is_valid(line, False):
        count_part1 += 1
    if password_is_valid(line, True):
        count_part2 += 1

print("Part 1: {}\n"
      "Part 2: {}".format(count_part1, count_part2))
