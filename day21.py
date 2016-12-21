import sys
import re

def swap_position(md, password, reverse):
    a = int(md.group(1))
    b = int(md.group(2))

    (password[a], password[b]) = (password[b], password[a])

def swap_letter(md, password, reverse):
    a = password.index(md.group(1))
    b = password.index(md.group(2))

    (password[a], password[b]) = (password[b], password[a])

def rotate_left(steps, password):
    password[0:] = password[steps:] + password[0:steps]

def rotate_right(steps, password):
    rotate_left(len(password) - steps, password)

def rotate(md, password, reverse):
    steps = int(md.group(2)) % len(password)
    if md.group(1) == ("left" if reverse else "right"):
        rotate_right(steps, password)
    else:
        rotate_left(steps, password)

def rotate_from_letter(md, password, reverse):
    pos = password.index(md.group(1))

    if reverse:
        if (pos & 1) == 1:
            steps = pos // 2 + 1
        else:
            if pos == 0:
                pos = len(password)
            steps = ((pos + len(password)) // 2 + 1) % len(password)
        rotate_left(steps, password)
    else:
        steps = pos + 1
        if pos >= 4:
            steps += 1
        rotate_right(steps, password)

def reverse_range(md, password, reverse):
    a = int(md.group(1))
    b = int(md.group(2))
    start = min(a, b)
    end = max(a, b)

    for i in range((end - start + 1) // 2):
        (password[start + i], password[end - i]) = \
            (password[end - i], password[start + i])

def move_position(md, password, reverse):
    a = int(md.group(1))
    b = int(md.group(2))
    if reverse:
        (a, b) = (b, a)
    password.insert(b, password.pop(a))

operations = [
    (re.compile(r'swap position ([0-9]+) with position ([0-9]+)$'),
     swap_position),
    (re.compile(r'swap letter (.) with letter (.)$'),
     swap_letter),
    (re.compile(r'rotate (left|right) ([0-9]+) steps?$'),
     rotate),
    (re.compile(r'rotate based on position of letter (.)$'),
     rotate_from_letter),
    (re.compile(r'reverse positions ([0-9]+) through ([0-9]+)$'),
     reverse_range),
    (re.compile(r'move position ([0-9]+) to position ([0-9]+)$'),
     move_position),
]

if len(sys.argv) > 1:
    password = list(sys.argv[1])
else:
    password = list("abcdefgh")

steps = []

for line in sys.stdin:
    for regexp, func in operations:
        md = regexp.match(line)
        if md:
            steps.append((md, func))
            break
    else:
        print("Invalid line: " + line.rstrip(), file=sys.stderr)
        sys.exit(1)

for md, func in steps:
    func(md, password, False)
print("Part 1: " + "".join(password))

password = list("fbgdceah")

for md, func in reversed(steps):
    func(md, password, True)
print("Part 2: " + "".join(password))
