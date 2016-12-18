import sys

def next_row(previous_row):
    def get_tile_type(position):
        if position == 0:
            return previous_row[1]
        elif position == len(previous_row) - 1:
            return previous_row[len(previous_row) - 2]
        elif previous_row[position - 1] == previous_row[position + 1]:
            return "."
        else:
            return "^"

    return "".join(get_tile_type(position)
                   for position in range(len(previous_row)))

def is_safe(ch):
    return ch == "."

if len(sys.argv) > 1:
    row = sys.argv[1]
else:
    row = sys.stdin.readline().rstrip()

n_traps = 0

for i in range(40):
    n_traps += sum(is_safe(ch) for ch in row)
    row = next_row(row)

print("Part 1:", n_traps)
