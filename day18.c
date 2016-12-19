#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <inttypes.h>
#include <string.h>

#define BITS_IN_PART (sizeof (uint64_t) * 8)

static int
get_n_parts(int row_length)
{
        return ((row_length + BITS_IN_PART - 1) / BITS_IN_PART);
}

static void
get_next_row(const uint64_t *prev_row,
             uint64_t *next_row,
             int row_length)
{
        int n_parts = get_n_parts(row_length);
        int i;

        for (i = 1; i <= n_parts; i++) {
                next_row[i] = (((prev_row[i] << 1) |
                                (prev_row[i - 1] >> (BITS_IN_PART - 1))) ^
                               ((prev_row[i] >> 1) |
                                (prev_row[i + 1] << (BITS_IN_PART - 1))));
        }

        next_row[n_parts] &= (UINT64_MAX >>
                              (BITS_IN_PART - (row_length % BITS_IN_PART)));
}

static uint64_t
solve(const uint64_t *start_row, int row_length, int n_rows)
{
        int n_parts = get_n_parts(row_length);
        uint64_t row_a[n_parts + 2], row_b[n_parts + 2];
        uint64_t *prev_row = row_a, *next_row = row_b, *tmp;
        uint64_t n_safe_spots = 0;
        int n_traps;
        int i, j;

        memcpy(prev_row + 1, start_row, sizeof *prev_row * n_parts);
        row_a[0] = row_a[n_parts + 1] = 0;
        row_b[0] = row_b[n_parts + 1] = 0;

        for (i = 0; i < n_rows; i++) {
                n_traps = 0;
                for (j = 0; j < n_parts; j++)
                        n_traps += __builtin_popcountl(prev_row[j + 1]);
                n_safe_spots += row_length - n_traps;

                get_next_row(prev_row, next_row, row_length);

                tmp = prev_row;
                prev_row = next_row;
                next_row = tmp;
        }

        return n_safe_spots;
}

int
main(int argc, char **argv)
{
        int row_length;
        uint64_t *row;
        const char *puzzle_input;
        int n_rows, n_parts;
        int i;

        if (argc < 2 || argc > 4) {
                fprintf(stderr, "usage: <puzzle input> [n_rows]\n");
                return EXIT_FAILURE;
        }

        puzzle_input = argv[1];
        row_length = strlen(puzzle_input);
        n_parts = get_n_parts(row_length);

        row = alloca(sizeof *row * n_parts);
        memset(row, 0, sizeof *row * n_parts);

        for (i = 0; puzzle_input[i]; i++) {
                row[i / (sizeof *row * 8)] |=
                        ((uint64_t) (puzzle_input[i] == '^')) <<
                        (i % (sizeof *row * 8));
        }

        if (argc >= 3) {
                n_rows = strtol(argv[2], NULL, 10);
                printf("%" PRIu64 "\n", solve(row, row_length, n_rows));
        } else {
                printf("Part 1: %" PRIu64 "\n", solve(row, row_length, 40));
                printf("Part 2: %" PRIu64 "\n", solve(row, row_length, 400000));
        }

        return EXIT_SUCCESS;
}
