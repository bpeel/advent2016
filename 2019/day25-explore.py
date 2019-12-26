#!/usr/bin/env python3

import sys
import subprocess
import re
import collections
import io

Room = collections.namedtuple('Room', ['name', 'exits', 'items'])

def read_room(fin):
    room_name = None
    items = []
    exits = []
    state = None

    for line in fin:
        print(line)
        md = re.match(r'^== (.*) ==$', line)
        if md:
            room_name = md.group(1)
            continue

        md = re.match(r'^Doors here lead:', line)
        if md:
            state = "doors"
            continue

        md = re.match(r'^Items here:', line)
        if md:
            state = "items"
            continue

        md = re.match(r'^- (.*)', line)
        if md:
            if state == "doors":
                exits.append(md.group(1))
            elif state == "items":
                items.append(md.group(1))
            continue

        md = re.match(r'^Command\?', line)
        if md:
            break

    # Going north here is special
    if room_name == "Security Checkpoint":
        exits.remove("north")

    return Room(room_name, exits, items)

def send(prog, command):
    print("<< {}".format(command))
    print(command, file=prog.stdin)

def wait_command(prog):
    for line in prog.stdout:
        print(line)
        if line.startswith("Command?"):
            break

def try_all_combos(prog, items):
    for mask in range(1 << len(items)):
        for i in range(len(items)):
            if (mask & (1 << i)) == 0:
                send(prog, "drop {}".format(items[i]))
                wait_command(prog)

        send(prog, "inv")
        wait_command(prog)
        send(prog, "north")

        room = read_room(prog.stdout)
        if room.name != "Security Checkpoint":
            return

        for i in range(len(items)):
            if (mask & (1 << i)) == 0:
                send(prog, "take {}".format(items[i]))
                wait_command(prog)

prog = subprocess.Popen(["./build/day25", "day25-input.txt"],
                        stdin=subprocess.PIPE,
                        stdout=subprocess.PIPE,
                        bufsize=1,
                        text=True,
                        encoding="utf-8")

room = read_room(prog.stdout)
rooms = {room.name : room}
stack = [(room, 0)]
bad_items = set(["molten lava",
                 "infinite loop",
                 "escape pod",
                 "photons",
                 "giant electromagnet"])
items = []
route = None

opposite_dir = {
    "north": "south",
    "south": "north",
    "east": "west",
    "west": "east"
}

while True:
    room, dir_to_try = stack.pop()

    if room.name == "Security Checkpoint":
        route = [r[0].exits[r[1] - 1] for r in stack]

    for item in room.items:
        if item in bad_items or item in items:
            continue
        send(prog, "take {}".format(item))
        wait_command(prog)
        items.append(item)

    for d in range(dir_to_try, len(room.exits)):
        send(prog, room.exits[d])
        next_room = read_room(prog.stdout)
        if next_room.name in rooms:
            print("** already been in {}".format(next_room.name))
            send(prog, opposite_dir[room.exits[d]])
            bt_room = read_room(prog.stdout)
            assert(bt_room.name == room.name)
        else:
            rooms[next_room.name] = next_room
            stack.append((room, d + 1))
            stack.append((next_room, 0))
            break
    else:
        if len(stack) < 1:
            break
        parent_room, last_dir = stack[-1]
        print("** backtracking to {}".format(parent_room.name))
        assert(last_dir > 0)
        send(prog, opposite_dir[parent_room.exits[last_dir - 1]])
        bt_room = read_room(prog.stdout)
        assert(bt_room.name == parent_room.name)

print(list(rooms.values()))
print(items)

for d in route:
    send(prog, d)
    room = read_room(prog.stdout)

assert(room.name == "Security Checkpoint")

try_all_combos(prog, items)
