#!/usr/bin/env python3

import sys

def fuel_for_mass(mass):
    return mass // 3 - 2

def all_fuel_for_mass(mass):
    total = 0

    while True:
        mass = fuel_for_mass(mass)

        if mass <= 0:
            return total

        total += mass

masses = list(int(line.rstrip()) for line in sys.stdin)

part1 = sum(fuel_for_mass(mass) for mass in masses)

print("Part 1: {}".format(part1))

part2 = sum(all_fuel_for_mass(mass) for mass in masses)

print("Part 2: {}".format(part2))
