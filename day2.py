import re
import sys

pos_x = -2
pos_y = 0

table = (
    "**1**"
    "*234*"
    "56789"
    "*ABC*"
    "**D**"
)

def valid(n, other):
    return abs(n) <= 2 and abs(other) <= 2 - abs(n)

for line in sys.stdin:
    for ch in line:
        if ch == "U" and valid(pos_y - 1, pos_x):
            pos_y -= 1
        elif ch == "D" and valid(pos_y + 1, pos_x):
            pos_y += 1
        elif ch == "L" and valid(pos_x - 1, pos_y):
            pos_x -= 1
        elif ch == "R" and valid(pos_x + 1, pos_y):
            pos_x += 1
    print(table[(pos_y + 2) * 5 + pos_x + 2], end='')

    
