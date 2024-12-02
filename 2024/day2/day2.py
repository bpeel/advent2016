import sys

data = [[int(x) for x in line.split()] for line in sys.stdin]

def direction(a, b):
    if a < b:
        return -1
    if a > b:
        return 1
    return 0

def safe(data):
    d = direction(data[0], data[1])

    if d == 0:
        return False

    last = data[0]

    for n in data[1:]:
        if direction(last, n) != d:
            return False

        if abs(n - last) > 3:
            return False

        last = n

    return True

part1 = sum(map(lambda n: int(safe(n)), data))

print(f"Part1 {part1}")

