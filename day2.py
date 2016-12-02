import re
import sys

pos_x = 1
pos_y = 1

for line in sys.stdin:
    for ch in line:
        if ch == "U" and pos_y > 0:
            pos_y -= 1
        elif ch == "D" and pos_y < 2:
            pos_y += 1
        elif ch == "L" and pos_x > 0:
            pos_x -= 1
        elif ch == "R" and pos_x < 2:
            pos_x += 1
    print(pos_y * 3 + pos_x + 1, end='')

    
