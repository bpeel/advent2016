import sys
import re

def has_pair(s):
    return re.search(r'(.)(?!\1)(.)\2\1', s) != None

def has_tls(s):
    for b in re.finditer(r'\[.*?\]', s):
        if has_pair(b.group(0)):
            return False

    return has_pair(s)

def has_ssl(s):
    outside = re.sub(r'\[.*?\]', '', s)

    for i in range(len(outside) - 2):
        if outside[i] != outside[i + 2] or outside[i + 1] == outside[i]:
            continue

        for c in re.finditer(r'\[.*?\]', s):
            c = c.group(0)

            if c.find(outside[i + 1] + outside[i] + outside[i + 1]) != -1:
                return True

    return False

lines = [line.rstrip() for line in sys.stdin]
print("Part 1", sum(map(has_tls, lines)))
print("Part 2", sum(map(has_ssl, lines)))
