import sys
import re
import hashlib

prefix = 'ugkcyxxp'

password = [" "] * 8

index = 0
mask = 0

while mask < 255:
    m = hashlib.md5()
    m.update(prefix.encode("utf-8"))
    m.update(str(index).encode("utf-8"))
    d = m.digest()

    index += 1

    if d[0:2] != b"\x00\x00" or d[2] & 0xf0 != 0x00:
        continue

    pos = d[2] & 0xf

    if pos >= 8 or (mask & (1 << pos)) != 0:
        continue

    mask |= (1 << pos)

    password[pos] = hex(d[3] >> 4)[2]
    print("".join(password))
