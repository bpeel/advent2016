import sys

frequencies = None

for line in sys.stdin:
    line = line.rstrip()

    if frequencies == None:
        frequencies = [{} for x in range(len(line))]

    for i, ch in enumerate(line):
        frequencies[i][ch] = frequencies[i].get(ch, 0) + 1

def get_word(frequencies, pick_letter_func):
    def get_letter(letters):
        return pick_letter_func(letters, key=lambda x: letters[x])

    return "".join(map(get_letter, frequencies))

print("Part 1: ", get_word(frequencies, max))
print("Part 2: ", get_word(frequencies, min))
