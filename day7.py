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
    for b in re.finditer(r'\[.*?\]', s):
        if has_pair(b.group(0)):
            print("Not " + s)
            return False

    return has_pair(s)

print(sum(map(valid, sys.stdin)))
