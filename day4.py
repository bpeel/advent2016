import sys
import re

total = 0

def decrypt(letter, shift):
    if letter < "a" or letter > "z":
        return letter
    else:
        return chr(((ord(letter[0]) - ord('a')) + shift) % 26 + ord("a"))

for line in sys.stdin:
    md = re.match(r'(.*)-([0-9]+)\[(.*)\]', line)
    name = md.group(1)
    sector = int(md.group(2))
    check = md.group(3)
    letters = {}

    for l in name:
        if l == "-":
            continue
        if l in letters:
            letters[l] += 1
        else:
            letters[l] = 1

    pairs = [(l, count) for l, count in letters.items()]
    pairs.sort(key = lambda x: (-x[1], x[0]))

    s = "".join(x[0] for x in pairs[:5])

    if check != s:
        continue
    
    total += sector
    
    dn = "".join(decrypt(letter, sector) for letter in name)

    print(dn, sector, check, s)

print(total)
