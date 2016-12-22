import sys
import re

class Node:
    pass

def nodes_from_io(input):
    line_re = re.compile(r'/dev/grid/node-x([0-9]+)-y([0-9]+) +'
                         r'([0-9]+)T +([0-9]+)T')

    for line in input:
        md = line_re.match(line)
        if not md:
            continue

        node = Node()
        node.x = int(md.group(1))
        node.y = int(md.group(2))
        node.size = int(md.group(3))
        node.used = int(md.group(4))
        yield node

def viable_pairs(nodes):
    for node_a in nodes:
        if node_a.used <= 0:
            continue
        for node_b in nodes:
            if node_a is node_b:
                continue 
            if node_a.used + node_b.used <= node_b.size:
                yield node_a, node_b

nodes = list(nodes_from_io(sys.stdin))

print("Part 1: ", sum(1 for _, _ in viable_pairs(nodes)))
