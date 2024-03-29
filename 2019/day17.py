#!/usr/bin/env python3

import sys
import subprocess
import collections
import time

Fork = collections.namedtuple('Fork', ['pos', 'connections', 'num'])

N_PROGS = 3

def parse_map(infile):
    return [line.rstrip() for line in infile if len(line) >= 2]

def get_map():
    if len(sys.argv) > 1:
        proc = subprocess.Popen(["./build/runner", "-a", sys.argv[1]],
                                stdout=subprocess.PIPE,
                                text=True,
                                encoding="utf-8")
        return parse_map(proc.stdout)
    else:
        return parse_map(sys.stdin)

def is_scaf(ch):
    return ch in "#^<>v"

def get_intersections(scaf_map, include_dead_ends=False):
    for y in range(len(scaf_map)):
        for x in range(len(scaf_map[y])):
            if not is_scaf(scaf_map[y][x]):
                continue

            count = 0

            if x > 0 and is_scaf(scaf_map[y][x - 1]):
                count += 1
            if x + 1 < len(scaf_map[y]) and is_scaf(scaf_map[y][x + 1]):
                count += 1
            if y > 0 and is_scaf(scaf_map[y - 1][x]):
                count += 1
            if y + 1 < len(scaf_map) and is_scaf(scaf_map[y + 1][x]):
                count += 1

            if count == 4 or (include_dead_ends and count == 1):
                yield (x, y)

def move_pos(pos, d):
    if d == 0:
        return pos[0], pos[1] - 1
    elif d == 1:
        return pos[0], pos[1] + 1
    elif d == 2:
        return pos[0] - 1, pos[1]
    else:
        return pos[0] + 1, pos[1]

def ninety_turn(dir_a, dir_b):
    return "LR"[(dir_b & 1) ^ (dir_a & 1) ^ (dir_a >> 1)]

def get_connection(scaf_map, forks, pos, last_dir):
    route = [1]

    while True:
        if pos in forks:
            return forks[pos], route, last_dir

        for d in range(4):
            if d == last_dir ^ 1:
                continue

            npos = move_pos(pos, d)
            nx, ny = npos

            if (nx >= 0 and nx < len(scaf_map[0]) and
                ny >= 0 and ny < len(scaf_map) and
                is_scaf(scaf_map[ny][nx])):
                if d == last_dir:
                    route[-1] += 1
                else:
                    route.append(ninety_turn(last_dir, d))
                    route.append(1)
                    last_dir = d
                    
                pos = npos
                break
        else:
            assert(False)

def get_graph(scaf_map):
    forks = {}

    for num, intersection in enumerate(get_intersections(scaf_map, True)):
        forks[intersection] = Fork(intersection, [None] * 4, num)

    for fork in forks.values():
        for d in range(4):
            npos = move_pos(fork.pos, d)
            nx, ny = npos
            if (nx >= 0 and nx < len(scaf_map[0]) and
                ny >= 0 and ny < len(scaf_map) and
                is_scaf(scaf_map[ny][nx])):
                connection = get_connection(scaf_map, forks, npos, d)
                fork.connections[d] = connection

    return list(forks.values())

def get_all_branches(graph):
    branches = 0

    for fork in graph:
        for i, connection in enumerate(fork.connections):
            if connection is None:
                continue
            branches |= 1 << (fork.num * 4 + i)

    return branches

def get_start(scaf_map):
    dirs = '^v<>'
    for y in range(len(scaf_map)):
        for x in range(len(scaf_map[y])):
            if scaf_map[y][x] in dirs:
                return x, y, dirs.index(scaf_map[y][x])

    return None

def add_rotation(route, dir_a, dir_b):
    if (dir_a & 10) != (dir_b & 10):
        route.append(ninety_turn(dir_a, dir_b))
    elif dir_a != dir_b:
        route.append("R")
        route.append("R")

def convert_route(graph, stack, last_dir):
    route = []

    for i in range(1, len(stack)):
        last_node = stack[i - 1][0]
        new_dir = stack[i - 1][1] - 1
        connection = last_node.connections[new_dir]
        new_route = connection[1]
        final_dir = connection[2]

        add_rotation(route, last_dir, new_dir)

        for part in new_route:
            if (len(route) > 0 and
                isinstance(route[-1], int) and
                isinstance(part, int)):
                route[-1] += part
            else:
                route.append(part)

        last_dir = final_dir

    return route

def get_visited_count(stack, node):
    count = 0
    for entry in stack:
        if entry[0] is node:
            count += 1

    return count

def get_routes(graph, start, start_dir):
    stack = [(start, 0, 0)]
    all_branches = get_all_branches(graph)

    while len(stack) > 0:
        node, dir_to_try, visited_mask = stack[-1]

        if visited_mask == all_branches:
            yield convert_route(graph, stack, start_dir)
            stack.pop()
            continue

        stack.pop()

        for d in range(dir_to_try, 4):
            if node.connections[d] is None:
                continue

            if (visited_mask & (1 << (node.num * 4 + d))) != 0:
                continue

            new_node = node.connections[d][0]

            stack.append((node, d + 1, visited_mask))

            visited_mask |= 1 << (node.num * 4 + d)
            final_d = node.connections[d][2]
            visited_mask |= 1 << (new_node.num * 4 + (final_d ^ 1))

            stack.append((new_node, 0, visited_mask))
            break

def count_copies_in_chunk(chunk, token):
    i = 0
    count = 0
    while i + len(token) <= len(chunk):
        for j in range(len(token)):
            if chunk[i + j] != token[j]:
                i += 1
                break
        else:
            count += 1
            i += len(token)

    return count

def count_copies(chunks, token):
    return sum(count_copies_in_chunk(chunk, token)
               for chunk in chunks
               if not isinstance(chunk, int))

def compress_route_part(chunks, depth):
    try:
        chunk_num = next(i for i in range(len(chunks))
                         if not isinstance(chunks[i], int))
    except StopIteration:
        return ([[]] * (N_PROGS - depth), chunks)

    first_chunk = chunks[chunk_num]

    for length in range(1, min(len(first_chunk), 10) + 1):
        token = first_chunk[0:length]

        nchunks = []
        had_uncompressed = False

        for chunk in chunks:
            if isinstance(chunk, int):
                nchunks.append(chunk)
                continue

            last_pos = 0

            i = 0
            while i + len(token) <= len(chunk):
                for j in range(len(token)):
                    if chunk[i + j] != token[j]:
                        i += 1
                        break
                else:
                    if i > last_pos:
                        nchunks.append(chunk[last_pos:i])
                        had_uncompressed = True

                    nchunks.append(depth)
                    i += len(token)
                    last_pos = i

            if last_pos < len(chunk):
                nchunks.append(chunk[last_pos:])
                had_uncompressed = True

        if depth + 1 < N_PROGS:
            sub = compress_route_part(nchunks, depth + 1)
            if sub is not None:
                return ([token] + sub[0], sub[1])
        elif not had_uncompressed:
            return ([token], nchunks)
                
    return None

def compress_route(route):
    return compress_route_part([route], 0)

scaf_map = get_map()

print("Part 1: ", sum(a * b for a, b in get_intersections(scaf_map)))

graph = get_graph(scaf_map)
start_data = get_start(scaf_map)
start_pos = (start_data[0], start_data[1])
start_dir = start_data[2]
start = next(x for x in graph if x.pos == start_pos)

start_time = time.monotonic()

for num, route in enumerate(get_routes(graph, start, start_dir)):
    print("\x1b[K  {}".format(num), end='')
    now_time = time.monotonic()
    if now_time > start_time + 1.0:
        print(" {}".format(num / (now_time - start_time)), end='')
    print("\r", end='')
    sys.stdout.flush()

    compression = compress_route(route)
    if compression is None:
        continue

    parts, order = compression

    print("\x1b[K===")
    print(",".join(chr(ord("A") + i) for i in order))
    for part in parts:
        print(",".join(str(x) for x in part))
