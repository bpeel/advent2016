#!/usr/bin/env python3

import re
import sys
import itertools
import copy
import functools
import math

class Moon:
    def __init__(self, desc):
        md = re.match(r'<x=(-?[0-9]+), *y=(-?[0-9]+), *z=(-?[0-9]+)>', desc)
        self.pos = [int(md.group(x + 1)) for x in range(3)]
        self.velocity = [0] * 3

    def get_total_energy(self):
        return (sum(abs(x) for x in self.pos) *
                sum(abs(x) for x in self.velocity))

def step_axis(moons, axis):
    for i in range(len(moons)):
        mi = moons[i]
        for j in range(i + 1, len(moons)):
            mj = moons[j]

            pi = mi.pos[axis]
            pj = mj.pos[axis]

            if pi < pj:
                mi.velocity[axis] += 1
                mj.velocity[axis] -= 1
            elif pi > pj:
                mi.velocity[axis] -= 1
                mj.velocity[axis] += 1

    for moon in moons:
        moon.pos[axis] += moon.velocity[axis]

def step(moons):
    for axis in range(3):
        step_axis(moons, axis)

def find_cycle_for_axis(base_moons, axis):
    moons = list(copy.copy(moon) for moon in base_moons)
    history = {}

    for step in itertools.count():
        pos = tuple((moon.pos[axis], moon.velocity[axis]) for moon in moons)

        if pos in history:
            return step

        history[pos] = step

        step_axis(moons, axis)

def get_lcm(a, b):
    return a * b // math.gcd(a, b)

moons = [Moon(line) for line in sys.stdin]

for i in range(1000):
    step(moons)

print("Part 1: {}".format(sum(moon.get_total_energy() for moon in moons)))

cycles = [find_cycle_for_axis(moons, axis) for axis in range(3)]

lcm = functools.reduce(get_lcm, cycles)

print("Part 2: {}".format(lcm))
