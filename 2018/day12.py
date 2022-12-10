#!/usr/bin/env python3

import sys
import re

INITIAL_STATE_RE = re.compile(r'initial state: ([#\.]+)')
RULE_RE = re.compile(r'([#\.]{5}) => ([#\.])')

class Rules:
    def __init__(self, lines):
        self.initial_state = set()
        self.rules = [False] * (1 << 5)

        for line in lines:
            md = INITIAL_STATE_RE.match(line)
            if md:
                for pos, ch in enumerate(md.group(1)):
                    if ch == '#':
                        self.initial_state.add(pos)

            md = RULE_RE.match(line)
            if md:
                ind = 0
                for pos, ch in enumerate(md.group(1)):
                    if ch == '#':
                        ind |= 1 << pos
                self.rules[ind] = md.group(2) == '#'

    def update_pos(state, next_state, pot):
        rule_num = 0
        for i in range(5):
            if i - 2 in state:
                rule_num |= 1 << i
        if self.rules[rule_num]:
            next_state.add(pot)
        else:
            next_state.discard(pot)


rules = Rules(sys.stdin)

state = set(rules.initial_state)
next_state = set()

for _ in range(20):
    next_state.clear()

    for pot in state:
        rules.update_pos(state, next_state, pot)

    state, next_state = next_state, state
    
