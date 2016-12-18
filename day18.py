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

def solve(row, n_rows):
    n_traps = 0

    for i in range(n_rows):
        n_traps += sum(is_safe(ch) for ch in row)
        row = next_row(row)

    return n_traps

if len(sys.argv) > 1:
    row = sys.argv[1]
else:
    row = sys.stdin.readline().rstrip()

print("Part 1:", solve(row, 40))
print("Part 2:", solve(row, 400000))
