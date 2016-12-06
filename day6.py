import sys

frequencies = None

for line in sys.stdin:
    line = line.rstrip()

    if frequencies == None:
        frequencies = [{} for x in range(len(line))]

    for i, ch in enumerate(line):
        frequencies[i][ch] = frequencies[i].get(ch, 0) + 1

def get_most_frequent(letters):
    return max(letters, key=lambda x: letters[x])

print("".join(map(get_most_frequent, frequencies)))
