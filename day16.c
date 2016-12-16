#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>

static char *
curve(const char *a, int a_length, int *result_length_out)
{
        int result_length = a_length * 2 + 1;
        char *result = malloc(result_length + 1);
        int i;

        memcpy(result, a, a_length);
        result[a_length] = '0';

        for (i = 0; i < a_length; i++) {
                result[a_length + 1 + i] =
                        (~a[a_length - i - 1] & 1) + '0';
        }

        result[result_length] = '\0';
        *result_length_out = result_length;

        return result;
}

static char *
checksum(const char *a, int a_length, int *result_length_out)
{
        int result_length = a_length / 2;
        char *result = malloc(result_length + 1);
        int i;

        for (i = 0; i < result_length; i++)
                result[i] = (~(a[i * 2] ^ a[i * 2 + 1]) & 1) + '0';

        result[result_length] = '\0';
        *result_length_out = result_length;

        return result;
}

static char *
solve(const char *start,
      int target_length)
{
        int a_length = strlen(start), b_length;
        char *a = strdup(start), *b;

        while (a_length < target_length) {
                b = curve(a, a_length, &b_length);
                free(a);
                a = b;
                a_length = b_length;
        }

        a_length = target_length;

        while ((a_length & 1) == 0) {
                b = checksum(a, a_length, &b_length);
                free(a);
                a = b;
                a_length = b_length;
        }

        return a;
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
