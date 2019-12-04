#include <stdio.h>
#include <limits.h>
#include <stdlib.h>
#include <stdbool.h>

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
has_repeat(int v)
{
        for (int digit = 0, div = 1; digit < N_DIGITS - 1; digit++, div *= 10) {
                int c = v / div % 10;
                int next = v / (div * 10) % 10;

                if (c == next)
                        return true;
        }

        return false;
}

static int
count_valid_passwords(int min, int max)
{
        int count = 0;

        for (int v = first_sequence(min);
             v <= max;
             v = next_sequence(v)) {
                if (has_repeat(v))
                        count++;
        }

        return count;
}

int
main(int argc, char **argv)
{
        if (argc != 3) {
                fprintf(stderr, "usage: day4 <min> <max>\n");
                return EXIT_FAILURE;
        }

        int count = count_valid_passwords(strtoul(argv[1], NULL, 10),
                                          strtoul(argv[2], NULL, 10));

        printf("Part 1: %i\n", count);

        return EXIT_SUCCESS;
}
