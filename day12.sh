#!/bin/bash

set -e

cat<<EOF > day12.c
#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>
#include <stdlib.h>
int
main(int argc, char **argv)
{
        int part;
        uint64_t a, b, c, d;

        for (part = 0; part < 2; part++) {
                printf("Part %i\n", part + 1);

                a = b = d = 0;
                c = part;

        asm("\n"
EOF

# Each instruction is padded out to 8 bytes with nops so that the jump
# instructions can just multiply the relative address by 8

sed -e 's/cpy \([0-9]\+\) \(.\)/mov $\1, %[\2]/' \
    -e 's/cpy \([a-z]\+\) \(.\)/mov %[\1], %[\2]/' \
    -e 's/\(inc\|dec\) \(.\)/\1 %[\2]/' \
    -e 's/jnz 0 .*/nop/' \
    -e 's/jnz [0-9]\+ \([+-]\?[0-9]\+\)/jmp . + 8 * \1/' \
    -e 's/jnz \(.\) \([+-]\?[0-9]\+\)/1: test %[\1],%[\1] ; jnz 1b + 8 * \2/' \
    -e 's/^/".align 8 ; /' \
    -e 's/$/ \\n"/' \
    >> day12.c

cat<<EOF >> day12.c
            ".align 8 ; nop\n"
            : [a] "+r" (a),
              [b] "+r" (b),
              [c] "+r" (c),
              [d] "+r" (d));
        printf("a = %" PRIu64 "\n"
               "b = %" PRIu64 "\n"
               "c = %" PRIu64 "\n"
               "d = %" PRIu64 "\n"
               "\n",
               a, b, c, d);
        }
        return EXIT_SUCCESS;
}
EOF

cc -Wall -o day12 day12.c
./day12
