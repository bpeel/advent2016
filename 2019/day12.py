#!/usr/bin/env python3

import re
import sys

class Moon:
    def __init__(self, desc):
        md = re.match(r'<x=(-?[0-9]+), *y=(-?[0-9]+), *z=(-?[0-9]+)>', desc)
        self.pos = [int(md.group(x + 1)) for x in range(3)]
        self.velocity = [0] * 3

    def get_total_energy(self):
        return (sum(abs(x) for x in self.pos) *
                sum(abs(x) for x in self.velocity))

def step(moons):
    for i in range(len(moons)):
        mi = moons[i]
        for j in range(i + 1, len(moons)):
            mj = moons[j]

            for axis in range(3):
                pi = mi.pos[axis]
                pj = mj.pos[axis]

                if pi < pj:
                    mi.velocity[axis] += 1
                    mj.velocity[axis] -= 1
                elif pi > pj:
                    mi.velocity[axis] -= 1
                    mj.velocity[axis] += 1

    for moon in moons:
        for axis in range(3):
            moon.pos[axis] += moon.velocity[axis]

moons = [Moon(line) for line in sys.stdin]

for i in range(1000):
    step(moons)

print("Part 1: {}".format(sum(moon.get_total_energy() for moon in moons)))

