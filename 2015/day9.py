import itertools
import sys
import re

def set_distance(routes, a, b, distance):
    routes.setdefault(a, {})[b] = distance

def route_length(routes, route):
    return sum(routes[route[i]][route[i + 1]] for i in range(len(route) - 1))

def find_route(routes, pick_func):
    route = pick_func(((x, route_length(routes, x))
                       for x in itertools.permutations(list(routes))),
                      key = lambda x: x[1])
    return " -> ".join(route[0]) + " (" + str(route[1]) + ")"

routes = {}

for line in sys.stdin:
    md = re.match(r'(.*?) to (.*?) = ([0-9]+)', line)
    distance = int(md.group(3))

    set_distance(routes, md.group(1), md.group(2), distance)
    set_distance(routes, md.group(2), md.group(1), distance)

print("Part 1: ", find_route(routes, min))
print("Part 2: ", find_route(routes, max))
