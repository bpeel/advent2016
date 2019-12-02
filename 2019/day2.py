#!/usr/bin/env python3

import sys

memory = [int(x.rstrip()) for x in sys.stdin.read().split(',')]

memory[1] = 12
memory[2] = 2

pos = 0

while True:
    opcode = memory[pos]

    if opcode == 99:
        break

    input_a = memory[memory[pos + 1]]
    input_b = memory[memory[pos + 2]]

    if opcode == 1:
        result = input_a + input_b
    elif opcode == 2:
        result = input_a * input_b
    else:
        print("unknown opcode {} at {}".format(opcode, pos), file=sys.stderr)
        sys.exit(1)

    memory[memory[pos + 3]] = result

    pos += 4

print("Part 1: {}".format(memory[0]))
