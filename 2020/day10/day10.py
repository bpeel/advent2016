import sys

adapters = (int(x.strip()) for x in sys.stdin)

threes = 1 # PC is always + 3
ones = 0
last = 0

for adapter in sorted(adapters):
    if adapter - last == 3:
        threes += 1
    elif adapter - last == 1:
        ones += 1
    last = adapter

print("threes = {}, ones = {}, part 1 = {}".format(threes, ones, threes * ones))
