import sys

def triangle(n):
    # Not really a triangle number because each new row adds 2 not 1
    return n * n

RANGE = 2
N_KEYS = triangle(RANGE) * 2 + RANGE * 2 + 1

pos_x = -2
pos_y = 0

def valid(n, other):
    return abs(n) <= RANGE and abs(other) <= RANGE - abs(n)

for line in sys.stdin:
    for ch in line:
        if ch == "U":
            if valid(pos_y - 1, pos_x):
                pos_y -= 1
        elif ch == "D":
            if valid(pos_y + 1, pos_x):
                pos_y += 1
        elif ch == "L":
            if valid(pos_x - 1, pos_y):
                pos_x -= 1
        elif ch == "R":
            if valid(pos_x + 1, pos_y):
                pos_x += 1

    if pos_y <= 0:
        num = triangle(RANGE + pos_y)
    else:
        num = N_KEYS - triangle(RANGE - pos_y + 1)

    num += pos_x + RANGE + 1 - abs(pos_y)

    print(hex(num)[2:], end='')

print()
