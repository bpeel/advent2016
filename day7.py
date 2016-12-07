import sys
import re
from itertools import permutations

count = 0

def has_pair(s):
    for b in re.finditer(r'(.)(.)\2\1', s):
        b = b.group(0)
        if b[0] != b[1]:
            return True

    return False

def valid(s):
    outside = re.sub(r'\[.*?\]', '', s)

    for i in range(len(outside) - 2):
        if outside[i] != outside[i + 2] or outside[i + 1] == outside[i]:
            continue

        for c in re.finditer(r'\[.*?\]', s):
            c = c.group(0)

            if re.search(outside[i + 1] + outside[i] + outside[i + 1], c):
                return True

    return False

print(sum(map(valid, sys.stdin)))
