import sys

left = []
right = []

for line in sys.stdin:
    parts = list(map(int, line.split()))
    left.append(parts[0])
    right.append(parts[1])

left.sort()
right.sort()

diffs = map(lambda a: abs(a[0] - a[1]), zip(left, right))
print(f"Part 1: {sum(diffs)}")

