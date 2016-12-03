import re
import sys

count = 0

triangles = []
num = 0
accum = []

for line in sys.stdin:
    parts = list(map(int, line.split()))

    if num == 0:
        accum = [[], [], []]

    for i in range(len(parts)):
        accum[i].append(parts[i])

    num += 1

    if num == 3:
        for p in accum:
            triangles.append(p)
        num = 0

for parts in triangles:
    good = True
    
    for i in range(len(parts)):
        s = 0
        for j in range(len(parts)):
            if i != j:
                s += parts[j]
        if s <= parts[i]:
            good = False
            break

    if good:
        count += 1

print(count)
    
