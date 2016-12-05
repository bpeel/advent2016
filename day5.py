import sys
import hashlib

prefix = 'ugkcyxxp'

part1 = ""
part2 = [" "] * 8

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

    if len(part1) < 8:
        part1 += hex(pos)[2]

    if pos < 8 and (mask & (1 << pos)) == 0:
        mask |= (1 << pos)

        part2[pos] = hex(d[3] >> 4)[2]

    print("\rPart 1: {:8s}  Part2: {} ".format(part1, "".join(part2)),
          end='')
    sys.stdout.flush()

print()
