#!/usr/bin/env python3

import sys
import itertools
import collections

def recipes():
    history = [3, 7]
    for n in history:
        yield n
    elf1 = 0
    elf2 = 1

    while True:
        r1 = history[elf1]
        r2 = history[elf2]
        new_sum = r1 + r2

        if new_sum > 9:
            history.append(new_sum // 10)
            yield history[-1]
        history.append(new_sum % 10)
        yield history[-1]

        elf1 = (elf1 + r1 + 1) % len(history)
        elf2 = (elf2 + r2 + 1) % len(history)

def collection_equal(a, b):
    bit = iter(b)
    for x in a:
        try:
            if x != next(bit):
                return False
        except StopIteration:
            return False

    try:
        next(bit)
    except StopIteration:
        return True

    return False

def find_in_recipes(to_find):
    tail = collections.deque()
    for i, recipe in enumerate(recipes()):
        if len(tail) >= len(to_find):
            tail.popleft()
        tail.append(recipe)

        if collection_equal(to_find, tail):
            return i - len(to_find) + 1

if len(sys.argv) != 2:
    print("usage: {} <n_recipes>", file=sys.stderr)
    sys.exit(1)

n_recipes = int(sys.argv[1])
part1_iterator = itertools.islice(recipes(), n_recipes, n_recipes + 10)
part1 = "".join(str(x) for x in part1_iterator)

print("Part 1: {}".format(part1))

print("Part 2: {}".format(find_in_recipes([int(x) for x in sys.argv[1]])))
