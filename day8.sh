#!/bin/bash

if [ -t 1 ]; then
    "$0" | python3
    exit 0;
fi

cat <<EOF
vars = {}
part2 = None
def get(name):
    return vars.get(name, 0)
def set(name, value):
    global part2
    if part2 is None or value > part2:
        part2 = value
    vars[name] = value
def inc(name, amount):
    set(name, get(name) + amount)
def dec(name, amount):
    set(name, get(name) - amount)
EOF

sed -r 's/([a-z]+) (inc|dec) (-?[0-9]+) if ([a-z]+) ([!><=]+ -?[0-9]+)/if get("\4") \5: \2("\1", \3)/'

echo 'print("Part 1: {}\nPart 2: {}".format(max(vars.values()), part2))'
