import sys
import re

def has_abba(s):
    return re.search(r'(.)(?!\1)(.)\2\1', s) != None

def has_tls(s):
    for md in re.finditer(r'\[.*?\]', s):
        if has_abba(md.group(0)):
            return False

    return has_abba(s)

def hypernet_has_bab(s, bab):
    for md in re.finditer(r'\[.*?\]', s):
        if md.group(0).find(bab) != -1:
            return True

    return False

def has_ssl(s):
    for outside in re.split(r'\[.*?\]', s):
        for i in range(len(outside) - 2):
            if outside[i] != outside[i + 2] or outside[i + 1] == outside[i]:
                continue

            if hypernet_has_bab(s,
                                outside[i + 1] + outside[i] + outside[i + 1]):
                return True

    return False

lines = [line.rstrip() for line in sys.stdin]
print("Part 1", sum(map(has_tls, lines)))
print("Part 2", sum(map(has_ssl, lines)))
