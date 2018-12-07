#!/usr/bin/env python3

import sys
import re

RULE_RE = re.compile(r'Step ([A-Z]) must be finished before '
                     r'step ([A-Z]) can begin.$')

class Step:
    def __init__(self, letter):
        self.letter = letter
        self.children = []
        self.num_parents = 0
        self.parents_completed = 0

class Instructions:
    def __init__(self):
        self.all_steps = {}
        self.root_steps = set()

    def get_step(self, letter):
        try:
            return self.all_steps[letter]
        except KeyError:
            step = Step(letter)
            self.all_steps[letter] = step
            self.root_steps.add(letter)
            return step

instructions = Instructions()

for line in sys.stdin:
    md = RULE_RE.match(line)
    parent = instructions.get_step(md.group(1))
    child = instructions.get_step(md.group(2))

    parent.children.append(child)
    instructions.root_steps.discard(child.letter)
    child.num_parents += 1

next_steps = set(instructions.root_steps)
history = []

while len(next_steps) > 0:
    next_step = min(next_steps)
    history.append(next_step)
    next_steps.remove(next_step)
    for child in instructions.get_step(next_step).children:
        child.parents_completed += 1
        if child.parents_completed >= child.num_parents:
            next_steps.add(child.letter)

print("Part 1: {}".format("".join(history)))
