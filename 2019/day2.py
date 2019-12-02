#!/usr/bin/env python3

import sys
import intcode

def run_program(memory, noun, verb):
    machine = intcode.Machine(memory)

    machine.write_memory(1, noun)
    machine.write_memory(2, verb)

    machine.run()

    return machine.read_memory(0)

def part2(memory):
    for noun in range(100):
        for verb in range(100):
            result = run_program(memory, noun, verb)
            if result == 19690720:
                return 100 * noun + verb

memory = [int(x.rstrip()) for x in sys.stdin.read().split(',')]

print("Part 1: {}".format(run_program(memory, 12, 2)))

print("Part 2: {}".format(part2(memory)))
