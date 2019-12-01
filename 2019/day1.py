#!/usr/bin/env python3

import sys

def fuel_for_mass(mass):
    return mass // 3 - 2

fuel_sum = sum(fuel_for_mass(int(line.rstrip())) for line in sys.stdin)

print(fuel_sum)
