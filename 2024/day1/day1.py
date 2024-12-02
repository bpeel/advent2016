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

counts = {}

for r in right:
    try:
        counts[r] += 1
    except KeyError:
        counts[r] = 1

def score(n):
    try:
        return counts[n] * n
    except KeyError:
        return 0

print(f"Part 2: {sum(map(score, left))}")


