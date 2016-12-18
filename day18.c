#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <inttypes.h>

static uint64_t
get_next_row(uint64_t prev_row, int row_length)
{
        return ((prev_row << 1) ^ (prev_row >> 1)) & ((1 << row_length) - 1);
}

static uint64_t
solve(uint64_t row, int row_length, int n_rows)
{
        uint64_t n_safe_spots = 0;
        int i;

        for (i = 0; i < n_rows; i++) {
                n_safe_spots += row_length - __builtin_popcountl(row);
                row = get_next_row(row, row_length);
        }

        return n_safe_spots;
}

int
main(int argc, char **argv)
{
        uint64_t row = 0;
        int row_length = 0;
        int n_rows;
        int ch;

        while (true) {
                ch = fgetc(stdin);

                if (ch == EOF)
                        break;

                if (ch == '.' || ch == '^') {
                        if (row_length >= sizeof row * 8) {
                                fprintf(stderr, "Row too long\n");
                                return EXIT_FAILURE;
                        }
                        row = (row << 1) | (ch == '^');
                        row_length++;
                }
        }

        if (argc >= 2) {
                n_rows = strtol(argv[1], NULL, 10);
                printf("%" PRIu64 "\n", solve(row, row_length, n_rows));
        } else {
                printf("Part 1: %" PRIu64 "\n", solve(row, row_length, 40));
                printf("Part 2: %" PRIu64 "\n", solve(row, row_length, 400000));
        }
}
