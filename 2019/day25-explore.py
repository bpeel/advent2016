#!/usr/bin/env python3

import sys
import subprocess
import re
import collections
import io

Room = collections.namedtuple('Room', ['name', 'exits', 'items'])

OPPOSITE_DIR = {
    "north": "south",
    "south": "north",
    "east": "west",
    "west": "east"
}

class NoCommandException(Exception):
    pass

def read_room(fin):
    room_name = None
    items = []
    exits = []
    state = None

    for line in fin:
        print(line.rstrip())
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
    count = 0

    for line in prog.stdout:
        print(line.rstrip())
        if line.startswith("Command?"):
            return

        count += 1

        if count > 100:
            raise NoCommandException("Too many lines with "
                                     "no command")

    raise NoCommandException("End of program reached while "
                             "waiting for prompt")

def get_all_items(prog, bad_items):
    room = read_room(prog.stdout)
    rooms = {room.name : room}
    stack = [(room, 0)]
    items = []
    route = None

    while True:
        room, dir_to_try = stack.pop()

        if room.name == "Security Checkpoint":
            route = [r[0].exits[r[1] - 1] for r in stack]

        for item in room.items:
            if item in bad_items or item in items:
                continue
            send(prog, "take {}".format(item))
            try:
                wait_command(prog)
            except NoCommandException:
                print("** Couldn’t pick up {}, restarting".format(item))
                bad_items.add(item)
                return None

            items.append(item)

        for d in range(dir_to_try, len(room.exits)):
            send(prog, room.exits[d])
            next_room = read_room(prog.stdout)
            if next_room.name is None:
                print("** Couldn’t move. Assuming the fault is “{}” and "
                      "restarting".format(items[-1]))
                bad_items.add(items[-1])
                return None
            if next_room.name in rooms:
                print("** already been in {}".format(next_room.name))
                send(prog, OPPOSITE_DIR[room.exits[d]])
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
            send(prog, OPPOSITE_DIR[parent_room.exits[last_dir - 1]])
            bt_room = read_room(prog.stdout)
            assert(bt_room.name == parent_room.name)

    return route, items

def try_all_combos(prog, items):
    last_state = (1 << len(items)) - 1

    for mask in range(last_state, -1, -1):
        diff = last_state ^ mask
        for i in range(len(items)):
            if (diff & (1 << i)) > 0:
                if last_state & (1 << i) == 0:
                    command = "take"
                else:
                    command = "drop"
                send(prog, "{} {}".format(command, items[i]))
                wait_command(prog)

        last_state = mask

        send(prog, "inv")
        wait_command(prog)
        send(prog, "north")

        room = read_room(prog.stdout)
        if room.name != "Security Checkpoint":
            return

bad_items = set()

while True:
    prog = subprocess.Popen(["./build/runner", "-a", "day25-input.txt"],
                            stdin=subprocess.PIPE,
                            stdout=subprocess.PIPE,
                            bufsize=1,
                            text=True,
                            encoding="utf-8")

    result = get_all_items(prog, bad_items)

    if result is not None:
        route, items = result
        break

    prog.kill()

for d in route:
    send(prog, d)
    room = read_room(prog.stdout)

assert(room.name == "Security Checkpoint")

try_all_combos(prog, items)
