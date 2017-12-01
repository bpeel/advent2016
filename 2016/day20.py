import sys
import re

ranges = []

for line in sys.stdin:
    md = re.match(r'([0-9]+)-([0-9]+)$', line)
    ranges.append((int(md.group(1)), int(md.group(2))))

first_free = 0

while True:
    last_first_free = first_free

    for r in ranges:
        if first_free >= r[0] and first_free <= r[1]:
            first_free = r[1] + 1

    if first_free == last_first_free:
        break

print(first_free)
    

    
