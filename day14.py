import hashlib
import sys
import re
from itertools import permutations

salt = "zpqevtbw"
#salt = "abc"

def get_hash(index):
    m = hashlib.md5()
    m.update(salt.encode("utf-8"))
    m.update(str(index).encode("utf-8"))
    h = m.hexdigest()

    for i in range(2016):
        m = hashlib.md5()
        m.update(h.encode("utf-8"))
        h = m.hexdigest()

    return h

hashes = [get_hash(x) for x in range(1001)]

index = 0
found = 0

triple = re.compile(r'(.)\1\1')
fivel = re.compile(r'(.)\1\1\1\1')

while True:
    md = triple.search(hashes.pop(0))
    if md:
        fivel = re.compile(md.group(1) * 5)
        got5 = False
        for h in hashes:
            if fivel.search(h):
                got5 = True
                break
        if got5:
            found += 1
            print(index, found)
            if found >= 64:
                break

    index += 1
    hashes.append(get_hash(index + len(hashes)))

print(index)
