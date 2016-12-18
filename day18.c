#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <inttypes.h>
#include <string.h>

#define BITS_IN_PART (sizeof (uint64_t) * 8)

struct history_entry {
        /* Cumulative safe spot count, excluding this row */
        uint64_t n_safe_spots;
        /* Over-allocated */
        uint64_t row[];
};

struct history {
        int size;
        int length;
        int n_parts;
        struct history_entry *entries;
};

static void
history_init(struct history *history,
             int n_parts)
{
        history->length = 0;
        history->size = 1;
        history->n_parts = n_parts;
        history->entries = malloc((sizeof (struct history_entry) +
                                   sizeof (uint64_t) * history->n_parts) *
                                  history->size);
}

static struct history_entry *
history_get_entry(struct history *history,
                  int index)
{
        size_t entry_size = (sizeof (struct history_entry) +
                             sizeof (uint64_t) * history->n_parts);

        return (struct history_entry *) ((uint8_t *) history->entries +
                                         entry_size * index);
}

static void
history_add(struct history *history,
            uint64_t n_safe_spots,
            const uint64_t *row)
{
        struct history_entry *entry;

        if (history->length >= history->size) {
                history->size *= 2;
                history->entries = realloc(history->entries,
                                           (sizeof (struct history_entry) +
                                            sizeof (uint64_t) *
                                            history->n_parts) *
                                           history->size);
        }

        entry = history_get_entry(history, history->length);
        entry->n_safe_spots = n_safe_spots;
        memcpy(entry->row, row, sizeof (uint64_t) * history->n_parts);

        history->length++;
}

static int
history_find(struct history *history,
             const uint64_t *row)
{
        struct history_entry *entry;
        int i;

        for (i = 0; i < history->length; i++) {
                entry = history_get_entry(history, i);

                if (!memcmp(entry->row,
                            row,
                            sizeof (uint64_t) * history->n_parts))
                        return i;
        }

        return -1;
}

static void
history_destroy(struct history *history)
{
        free(history->entries);
}

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

static int
n_safe_spots_in_row(int row_length,
                    int n_parts,
                    const uint64_t *row)
{
        int n_traps = 0, i;

        for (i = 0; i < n_parts; i++)
                n_traps += __builtin_popcountl(row[i]);

        return row_length - n_traps;
}

static uint64_t
calculate_cycle_safe_spots(struct history *history,
                           int cycle_start,
                           int row_length,
                           uint64_t n_rows)
{
        struct history_entry *cycle_start_entry =
                history_get_entry(history, cycle_start);
        struct history_entry *last_entry, *in_cycle_entry;
        uint64_t before_cycle, cycle_size, in_cycle_size;
        int cycle_n_rows;
        int in_cycle_pos;

        before_cycle = cycle_start_entry->n_safe_spots;
        last_entry = history_get_entry(history, history->length - 1);
        cycle_size = (last_entry->n_safe_spots - before_cycle +
                      n_safe_spots_in_row(row_length,
                                          history->n_parts,
                                          last_entry->row));
        cycle_n_rows = history->length - cycle_start;
        in_cycle_pos = cycle_start + (n_rows - cycle_start) % cycle_n_rows;
        in_cycle_entry = history_get_entry(history, in_cycle_pos);
        in_cycle_size = in_cycle_entry->n_safe_spots - before_cycle;

        return (before_cycle +
                n_rows / cycle_n_rows * cycle_size +
                in_cycle_size);
}

static uint64_t
solve(const uint64_t *start_row, int row_length, uint64_t n_rows)
{
        struct history history;
        int n_parts = get_n_parts(row_length);
        uint64_t row_a[n_parts + 2], row_b[n_parts + 2];
        uint64_t *prev_row = row_a, *next_row = row_b, *tmp;
        uint64_t n_safe_spots = 0;
        int cycle_start = -1, i;

        history_init(&history, n_parts);

        memcpy(prev_row + 1, start_row, sizeof *prev_row * n_parts);
        row_a[0] = row_a[n_parts + 1] = 0;
        row_b[0] = row_b[n_parts + 1] = 0;

        for (i = 0; i < n_rows; i++) {
                cycle_start = history_find(&history, prev_row + 1);
                if (cycle_start != -1) {
                        n_safe_spots = calculate_cycle_safe_spots(&history,
                                                                  cycle_start,
                                                                  row_length,
                                                                  n_rows);
                        break;
                }

                history_add(&history, n_safe_spots, prev_row + 1);

                n_safe_spots += n_safe_spots_in_row(row_length,
                                                    n_parts,
                                                    prev_row + 1);

                get_next_row(prev_row, next_row, row_length);

                tmp = prev_row;
                prev_row = next_row;
                next_row = tmp;
        }

        history_destroy(&history);

        return n_safe_spots;
}

int
main(int argc, char **argv)
{
        int row_length;
        uint64_t *row;
        const char *puzzle_input;
        uint64_t n_rows;
        int n_parts, i;

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
                n_rows = strtoull(argv[2], NULL, 10);
                printf("%" PRIu64 "\n", solve(row, row_length, n_rows));
        } else {
                printf("Part 1: %" PRIu64 "\n", solve(row, row_length, 40));
                printf("Part 2: %" PRIu64 "\n", solve(row, row_length, 400000));
        }

        return EXIT_SUCCESS;
}
