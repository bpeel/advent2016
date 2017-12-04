#!/usr/bin/python3

import sys

count = 0

def password_is_valid(password):
    words = {}
    for word in password.split():
        if word in words:
            return False
        words[word] = True

    return True

for line in sys.stdin:
    if password_is_valid(line):
        count += 1

print(count)
