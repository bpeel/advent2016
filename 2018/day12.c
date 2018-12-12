#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>
#include <stdint.h>
#include <inttypes.h>

struct bit_set {
        int first_bit;
        int n_longs;
        unsigned long *bits;
};

#define BITS_PER_LONG ((int) (sizeof (unsigned long)) * 8)

static void
bit_set_init(struct bit_set *bit_set)
{
        bit_set->first_bit = 0;
        bit_set->n_longs = 1;
        bit_set->bits = calloc(bit_set->n_longs, sizeof *bit_set->bits);
}

static void
bit_set_destroy(struct bit_set *bit_set)
{
        free(bit_set->bits);
}

static void
bit_set_clear_all(struct bit_set *bit_set)
{
        memset(bit_set->bits, 0, sizeof *bit_set->bits * bit_set->n_longs);
}

static bool
bit_set_test_bit(const struct bit_set *bit_set,
                 int pos)
{
        if (pos < bit_set->first_bit ||
            pos - bit_set->first_bit >= BITS_PER_LONG * bit_set->n_longs)
                return false;

        int bit_num = pos - bit_set->first_bit;

        return (bit_set->bits[bit_num / BITS_PER_LONG] &
                (1ul << (bit_num % BITS_PER_LONG)));
}

static void
bit_set_set_bit(struct bit_set *bit_set,
                int pos)
{
        if (pos < bit_set->first_bit) {
                int extra_bits = bit_set->first_bit - pos;
                int extra_longs = ((extra_bits + BITS_PER_LONG - 1) /
                                   BITS_PER_LONG);

                bit_set->bits = realloc(bit_set->bits,
                                        (bit_set->n_longs + extra_longs) *
                                        sizeof *bit_set->bits);

                memmove(bit_set->bits + extra_longs,
                        bit_set->bits,
                        bit_set->n_longs * sizeof *bit_set->bits);
                memset(bit_set->bits, 0, extra_longs * sizeof *bit_set->bits);

                bit_set->first_bit -= extra_longs * BITS_PER_LONG;
                bit_set->n_longs += extra_longs;
        } else if (pos >= (bit_set->first_bit +
                           bit_set->n_longs * BITS_PER_LONG)) {
                int extra_bits = pos - (bit_set->first_bit +
                                        bit_set->n_longs * BITS_PER_LONG) + 1;
                int extra_longs = ((extra_bits + BITS_PER_LONG - 1) /
                                   BITS_PER_LONG);
                bit_set->bits = realloc(bit_set->bits,
                                        (bit_set->n_longs + extra_longs) *
                                        sizeof *bit_set->bits);
                memset(bit_set->bits + bit_set->n_longs,
                       0,
                       extra_longs * sizeof *bit_set->bits);
                bit_set->n_longs += extra_longs;
        }

        int bit_pos = pos - bit_set->first_bit;

        bit_set->bits[bit_pos / BITS_PER_LONG] |=
                1ul << (bit_pos % BITS_PER_LONG);
}

static void
bit_set_for_each_bit(const struct bit_set *bit_set,
                     void (* func)(int pos, void *data),
                     void *data)
{
        int first_bit = bit_set->first_bit;
        int n_longs = bit_set->n_longs;

        for (int long_num = 0; long_num < n_longs; long_num++) {
                unsigned long n = bit_set->bits[long_num];

                while (n) {
                        int bit_num = ffsl(n) - 1;
                        func(first_bit + long_num * BITS_PER_LONG + bit_num,
                             data);
                        n &= ~(1ul << bit_num);
                }
        }
}

static bool
read_state(FILE *in,
           bool *rules,
           struct bit_set *set)
{
        static const char header[] = "initial state: ";

        for (const char *p = header; *p; p++) {
                if (fgetc(in) != *p)
                        return false;
        }

        int pos = 0;

        while (true) {
                switch (fgetc(in)) {
                case '#':
                        bit_set_set_bit(set, pos);
                        break;
                case '.':
                        break;
                case '\n':
                        goto got_initial_state;
                default:
                        return false;
                }

                pos++;
        }

got_initial_state:

        if (fgetc(in) != '\n')
                return false;

        while (true) {
                int rule_index = 0;

                for (int i = 0; i < 5; i++) {
                        switch (fgetc(in)) {
                        case '.':
                                break;
                        case '#':
                                rule_index |= 1 << i;
                                break;
                        case EOF:
                                if (i == 0)
                                        return true;
                                return false;
                        default:
                                return false;
                        }
                }

                for (const char *p = " => "; *p; p++) {
                        if (fgetc(in) != *p)
                                return false;
                }

                switch (fgetc(in)) {
                case '.':
                        break;
                case '#':
                        rules[rule_index] = true;
                        break;
                default:
                        return false;
                }

                if (fgetc(in) != '\n')
                        return false;
        }
}

struct iterate_closure {
        const bool *rules;
        const struct bit_set *set_a;
        struct bit_set *set_b;
        int last_bit;
};

static void
iterate_cb(int bit_pos,
           void *user_data)
{
        struct iterate_closure *data = user_data;

        int start_bit = bit_pos - 2;

        if (start_bit < data->last_bit + 1)
                start_bit = data->last_bit + 1;

        for (int test_bit = start_bit; test_bit <= bit_pos + 2; test_bit++) {
                int rule_index = 0;

                for (int i = 0; i < 5; i++) {
                        if (bit_set_test_bit(data->set_a,
                                             test_bit + i - 2))
                                rule_index |= (1 << i);
                }

                if (data->rules[rule_index])
                        bit_set_set_bit(data->set_b, test_bit);
        }

        data->last_bit = bit_pos + 2;
}

static void
iterate(const bool *rules,
        const struct bit_set *set_a,
        struct bit_set *set_b)
{
        struct iterate_closure data = {
                .rules = rules,
                .set_a = set_a,
                .set_b = set_b,
                .last_bit = INT_MIN,
        };

        bit_set_for_each_bit(set_a, iterate_cb, &data);
}

static void
calc_sum_cb(int bit, void *user_data)
{
        uint64_t *sum = user_data;

        *sum += bit;
}

static uint64_t
calc_sum(const struct bit_set *set)
{
        uint64_t sum = 0;

        bit_set_for_each_bit(set, calc_sum_cb, &sum);

        return sum;
}

static uint64_t
run_iterations(const bool *rules,
               const struct bit_set *initial_state,
               uint64_t n_iterations)
{
        struct bit_set bit_sets[2];
        struct bit_set *set_a = bit_sets + 0;
        struct bit_set *set_b = bit_sets + 1;
        struct bit_set *tmp_set;

        *set_a = *initial_state;
        set_a->bits = malloc(initial_state->n_longs * sizeof *set_a->bits);
        memcpy(set_a->bits,
               initial_state->bits,
               initial_state->n_longs * sizeof *set_a->bits);
        bit_set_init(set_b);

        uint64_t sum = 0;

        for (uint64_t i = 0; i < n_iterations; i++) {
                iterate(rules, set_a, set_b);

                tmp_set = set_a;
                set_a = set_b;
                set_b = tmp_set;

                bit_set_clear_all(set_b);
        }

        sum = calc_sum(set_a);

        bit_set_destroy(set_a);
        bit_set_destroy(set_b);

        return sum;
}

int
main(int argc, char **argv)
{
        struct bit_set initial_state;
        bool rules[1 << 5] = { false };
        int ret = EXIT_SUCCESS;

        bit_set_init(&initial_state);

        if (read_state(stdin, rules, &initial_state)) {
                printf("Part 1: %" PRIu64 "\n",
                       run_iterations(rules, &initial_state, 20));
                printf("Part 2: %" PRIu64 "\n",
                       run_iterations(rules,
                                      &initial_state,
                                      UINT64_C(50000000000)));
        } else {
                fprintf(stderr, "error reading input\n");
                ret = EXIT_FAILURE;
        }

        bit_set_destroy(&initial_state);

        return ret;
}
