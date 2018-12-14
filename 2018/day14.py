#!/usr/bin/env python3

import sys

def get_next_ten(after):
    history = [3, 7]
    elf1 = 0
    elf2 = 1

    while len(history) < after + 10:
        r1 = history[elf1]
        r2 = history[elf2]
        new_sum = r1 + r2

        if new_sum > 9:
            history.append(new_sum // 10)
        history.append(new_sum % 10)

        elf1 = (elf1 + r1 + 1) % len(history)
        elf2 = (elf2 + r2 + 1) % len(history)

    return "".join(str(x) for x in history[after : after + 10])

if len(sys.argv) != 2:
    print("usage: {} <n_recipes>", file=sys.stderr)
    sys.exit(1)

n_recipes = int(sys.argv[1])
print("Part 1: {}".format(get_next_ten(n_recipes)))
