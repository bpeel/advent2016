#!/usr/bin/env python3

import sys
import re

class Cart:
    def __init__(self, x, y, d):
        self.x = x
        self.y = y
        self.d = d
        self.turn = 0

class State:
    def __init__(self, lines):
        width = None

        self.lines = []
        self.carts = []
        self.had_collision = False

        for y, line in enumerate(lines):
            self.lines.append(re.sub(r'[<>^]',
                                     lambda x: '|' if x == '^' else '-',
                                     line))

            for x, ch in enumerate(line):
                if ch == '^':
                    d = 0
                elif ch == '<':
                    d = 3
                elif ch == '>':
                    d = 1
                elif ch == 'v':
                    d = 2
                else:
                    continue
                self.carts.append(Cart(x, y, d))

    def take_turn(self):
        sorted_carts = list(sorted(self.carts, key = lambda cart: cart.y))

        for cart in sorted_carts:
            ch = self.lines[cart.y][cart.x]
            if ch == '/':
                cart.d ^= 1
            elif ch == '\\':
                cart.d = 3 - cart.d
            elif ch == '+':
                if cart.turn == 0:
                    cart.d = (cart.d + 3) % 4
                elif cart.turn == 2:
                    cart.d = (cart.d + 1) % 4
                else:
                    assert(cart.turn == 1)
                cart.turn = (cart.turn + 1) % 3

            if cart.d == 0:
                cart.y -= 1
            elif cart.d == 1:
                cart.x += 1
            elif cart.d == 2:
                cart.y += 1
            elif cart.d == 3:
                cart.x -= 1
            else:
                assert(false)

            for other_cart in self.carts:
                if other_cart is cart:
                    continue

                if other_cart.x == cart.x and other_cart.y == cart.y:
                    if not self.had_collision:
                        print("Part 1: {},{}".format(cart.x, cart.y))
                        self.had_collision = True
                    self.carts.remove(other_cart)
                    self.carts.remove(cart)
                    break

    def find_last_pos(self):
        while True:
            self.take_turn()

            if len(self.carts) <= 1:
                return (self.carts[0].x, self.carts[0].y)

last_pos = State(sys.stdin).find_last_pos()

print("Part 2: {}".format(",".join(str(x) for x in last_pos)))
