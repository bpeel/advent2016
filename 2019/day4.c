#include <stdio.h>
#include <limits.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#define N_DIGITS 6

static int
repeat_digit(int digit, int count)
{
        int sum = 0;

        for (int i = 0; i < count; i++) {
                sum += digit;
                digit *= 10;
        }

        return sum;
}

static int
next_sequence(int v)
{
        for (int digit = 0, div = 1; digit < N_DIGITS; digit++, div *= 10) {
                int c = v / div % 10;

                if (c < 9) {
                        int rest = v / (div * 10) * (div * 10);
                        return rest + repeat_digit(c + 1, digit + 1);
                }
        }

        return INT_MAX;
}

static int
first_sequence(int v)
{
        int div = 1;

        for (int i = 0; i < N_DIGITS - 1; i++)
                div *= 10;

        for (int i = 0; i < N_DIGITS - 1; i++, div /= 10) {
                int c = v / div % 10;
                int next = v / (div / 10) % 10;

                if (next < c) {
                        int rest = v / div * div;
                        return rest + repeat_digit(c, N_DIGITS - 1 - i);
                }
        }

        return v;
}

static bool
has_repeat(int v, int part)
{
        for (int digit = 0, div = 1; digit < N_DIGITS - 1;) {
                int c = v / div % 10;
                int repeats = 1, next_div = div;

                while (digit + repeats < N_DIGITS) {
                        next_div *= 10;

                        int next = v / next_div % 10;

                        if (next != c)
                                break;

                        repeats++;
                }

                if (part >= 2) {
                        if (repeats == 2)
                                return true;
                } else if (repeats >= 2) {
                        return true;
                }

                for (int i = 0; i < repeats; i++) {
                        digit++;
                        div *= 10;
                }
        }

        return false;
}

static void
count_valid_passwords(int min, int max, int *results)
{
        memset(results, 0, 2 * sizeof *results);

        for (int v = first_sequence(min);
             v <= max;
             v = next_sequence(v)) {
                for (int i = 0; i < 2; i++) {
                        if (has_repeat(v, i + 1))
                                results[i]++;
                }
        }
}

int
main(int argc, char **argv)
{
        if (argc != 3) {
                fprintf(stderr, "usage: day4 <min> <max>\n");
                return EXIT_FAILURE;
        }

        int min = strtoul(argv[1], NULL, 10);
        int max = strtoul(argv[2], NULL, 10);
        int count[2];

        count_valid_passwords(min, max, count);

        for (int i = 0; i < 2; i++)
                printf("Part %i: %i\n", i + 1, count[i]);

        return EXIT_SUCCESS;
}
