#!/usr/bin/env python3

import sys
import re

class Node:
    def __init__(self, value):
        self.value = value
        self.prev = self
        self.next = self

    def insert_after(self, node):
        node.prev = self
        node.next = self.next
        self.next = node
        node.next.prev = node

    def remove(self):
        self.prev.next = self.next;
        self.next.prev = self.prev;
        self.next = None
        self.prev = None

def listify(input):
    head = Node(None)

    for ch in input:
        head.prev.insert_after(Node(ch))

    return head

def reduce_polymer(input):
    length = len(input)
    polymer = listify(input)
    n = polymer.next

    while n is not polymer and n.next is not polymer:
        a = n.value
        b = n.next.value
        if a.isupper() != b.isupper() and a.lower() == b.lower():
            n = n.next.next
            n.prev.remove()
            n.prev.remove()
            length -= 2
            if n.prev is not polymer:
                n = n.prev
        else:
            n = n.next

    return length

def strip_reduce_polymer(input, ch):
    return reduce_polymer(re.sub(ch, "", input, flags=re.IGNORECASE))

input = sys.stdin.read().rstrip()

print("Part 1: {}".format(reduce_polymer(input)))

part2 = min(((chr(ch), strip_reduce_polymer(input, chr(ch)))
             for ch in range(ord("a"), ord("z") + 1)),
            key=lambda x: x[1])

print("Part 2: {} {}".format(*part2))
