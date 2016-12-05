#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <openssl/md5.h>

#define PASSWORD_LENGTH 8

static void
init_password(char *password)
{
        memset(password, ' ', PASSWORD_LENGTH);
        password[PASSWORD_LENGTH] = 0;
}

static char
hexdigit(int digit)
{
        return digit >= 10 ? digit - 10 + 'a' : digit + '0';
}

int
main(int argc, char **argv)
{
        int num_len, part1_len = 0;
        uint8_t hash[MD5_DIGEST_LENGTH];
        char part1[PASSWORD_LENGTH + 1];
        char part2[PASSWORD_LENGTH + 1];
        const char *room_name;
        char numbuf[32];
        MD5_CTX md5_ctx, md5_ctx_base;
        int mask = 0;
        int num = 0;
        int pos;

        if (argc != 2) {
                fprintf(stderr, "usage: day5 <room_name>\n");
                return EXIT_FAILURE;
        }

        room_name = argv[1];

        MD5_Init(&md5_ctx_base);
        MD5_Update(&md5_ctx_base, room_name, strlen(room_name));

        init_password(part1);
        init_password(part2);

        while (mask < (1 << PASSWORD_LENGTH) - 1) {
                num_len = sprintf(numbuf, "%i", num);
                num++;

                md5_ctx = md5_ctx_base;
                MD5_Update(&md5_ctx, numbuf, num_len);
                MD5_Final(hash, &md5_ctx);

                if (hash[0] || hash[1] || (hash[2] >> 4))
                        continue;

                pos = hash[2] & 0x0f;

                if (part1_len < PASSWORD_LENGTH)
                        part1[part1_len++] = hexdigit(pos);

                if (pos < PASSWORD_LENGTH && (mask & (1 << pos)) == 0) {
                        mask |= 1 << pos;
                        part2[pos] = hexdigit(hash[3] >> 4);
                }

                printf("\rPart 1: %s Part 2: %s ", part1, part2);
                fflush(stdout);
        }

        fputc('\n', stdout);

        return 0;
}
