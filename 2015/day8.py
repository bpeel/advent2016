import sys
import re

part1 = 0
part2 = 0

for line in sys.stdin:
    line = line.rstrip()

    part1 += sum(len(md.group(0)) - 1 for md in
                 re.finditer(r'\\(x[0-9a-f]{2}|\\|")', line)) + 2
    part2 += 2 + sum(1 for _ in re.finditer(r'["\\]', line))

print("Part 1: ", part1)
print("Part 2: ", part2)

