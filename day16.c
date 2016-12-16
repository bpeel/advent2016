#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>

static void
curve_step(char *buf,
           int a_length)
{
        int i;

        buf[a_length] = '0';

        for (i = 0; i < a_length; i++) {
                buf[a_length + 1 + i] =
                        (~buf[a_length - i - 1] & 1) + '0';
        }
}

static char *
curve(const char *a,
      int a_length,
      int target_length)
{
        int result_length;
        char *result;

        result_length = a_length;

        do {
                result_length = result_length * 2 + 1;
        } while (result_length < target_length);

        result = malloc(result_length + 1);
        memcpy(result, a, a_length);

        do {
                curve_step(result, a_length);
                a_length = a_length * 2 + 1;
        } while (a_length < target_length);

        result[result_length] = '\0';

        return result;
}

static int
checksum_part(const char *part, int part_length)
{
        int i, result = 1;

        for (i = 0; i < part_length; i += 2)
                result ^= ~(part[i] ^ part[i + 1]) & 1;

        return result;
}

static char *
checksum(const char *a, int a_length)
{
        int result_length;
        char *result;
        int part_length;
        int i;

        if (a_length & 1)
                return strndup(a, a_length);

        result_length = a_length;
        part_length = 1;

        do {
                result_length /= 2;
                part_length *= 2;
        } while ((result_length & 1) == 0);

        result = malloc(result_length + 1);

        for (i = 0; i < result_length; i++) {
                result[i] = (checksum_part(a + i * part_length, part_length) +
                             '0');
        }

        result[result_length] = '\0';

        return result;
}

static char *
solve(const char *start,
      int target_length)
{
        char *curved, *checksummed;

        curved = curve(start, strlen(start), target_length);
        checksummed = checksum(curved, target_length);

        free(curved);

        return checksummed;
}

int
main(int argc, char **argv)
{
        const char *puzzle_input = "01111001100111011";
        char *result;

        if (argc > 1)
                puzzle_input = argv[1];

        result = solve(puzzle_input, 272);
        printf("Part 1: %s\n", result);
        free(result);

        result = solve(puzzle_input, 35651584);
        printf("Part 2: %s\n", result);
        free(result);

        return 0;
}
