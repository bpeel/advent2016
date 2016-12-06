import sys
import re

def get_function(sources, exp):
    md = re.match(r'([0-9]+)$', exp)
    if md:
        value = int(md.group(1))
        return lambda: value
    md = re.match(r'([a-z]+)$', exp)
    if md:
        value = md.group(1)
        return lambda: sources[value]()
    md = re.match(r'([a-z0-9]+) ([A-Z]+) ([a-z0-9]+)$', exp)
    if md:
        a = get_function(sources, md.group(1))
        b = get_function(sources, md.group(3))
        op = md.group(2)

        if op == "AND":
            return lambda: a() & b()
        elif op == "OR":
            return lambda: a() | b()
        elif op == "LSHIFT":
            return lambda: a() << b()
        elif op == "RSHIFT":
            return lambda: a() >> b()
    md = re.match(r'NOT ([a-z0-9]+)$', exp)
    if md:
        a = get_function(sources, md.group(1))
        return lambda: ~a()
    raise ValueError("Unknown expression: " + exp)

sources = {}

for line in sys.stdin:
    md = re.match(r'(.*?) -> ([a-z]+)$', line)
    sources[md.group(2)] = get_function(sources, md.group(1))
        
print("Part 1: ", sources["a"]())
