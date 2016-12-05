import sys
import re
import hashlib

prefix = 'ugkcyxxp'
password = ''

index = 0

for i in range(8):
    while True:
        m = hashlib.md5()
        m.update(prefix.encode("utf-8"))
        m.update(str(index).encode("utf-8"))
        d = m.digest()

        index += 1

        if d[0:2] == b"\x00\x00" and d[2] & 0xf0 == 0x00:
            break

    password += hex(d[2] & 0xf)[2]
    print(password)
