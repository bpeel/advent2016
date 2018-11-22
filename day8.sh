#!/bin/bash

if [ -t 1 ]; then
    "$0" | python3
    exit 0;
fi

cat <<EOF
vars = {}
def get(name):
    return vars.get(name, 0)
def inc(name, amount):
    vars[name] = get(name) + amount
def dec(name, amount):
    vars[name] = get(name) - amount
EOF

sed -r 's/([a-z]+) (inc|dec) (-?[0-9]+) if ([a-z]+) ([!><=]+ -?[0-9]+)/if get("\4") \5: \2("\1", \3)/'


echo 'print(max(vars.values()))'
