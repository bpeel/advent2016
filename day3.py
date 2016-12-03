import re
import sys

count = 0

for line in sys.stdin:
    parts = list(map(int, line.split()))

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
    
