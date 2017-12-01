import sys
import re
import hashlib

data = '01111001100111011'
a = int(data, 2)
a_len = len(data)
TARGET_LEN = 272

while a_len < TARGET_LEN:
    b = 0

    for i in range(a_len):
        b = (b << 1) | (((~a) >> i) & 1)

    a = (a << (a_len + 1)) | b

    a_len = a_len * 2 + 1

    print(a_len, bin(a))

a >>= a_len - TARGET_LEN
a_len = TARGET_LEN
print("Doing", a_len, bin(a))

while (a_len & 1) == 0:
    b = 0
    for i in range(a_len // 2):
        nib = (a >> (a_len - i * 2 - 2)) & 3
        b |= ((~(nib & 1) ^ (nib >> 1)) & 1) << (a_len // 2 - i - 1)
    a = b
    a_len //= 2

    print(a_len, bin(a))

print()
a = bin(a)[2:]
a = "0" * (a_len - len(a)) + a
print(a)
