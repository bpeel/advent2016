#include <stdint.h>
#include <stdlib.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <limits.h>

static size_t
reduce_polymer(size_t polymer_length,
               const char *polymer)
{
        char *stack = malloc(polymer_length);
        char *stack_pos = stack;

        for (size_t i = 0; i < polymer_length; i++) {
                if (stack_pos > stack &&
                    abs(stack_pos[-1] - polymer[i]) == 'a' - 'A')
                        stack_pos--;
                else
                        *(stack_pos++) = polymer[i];
        }

        free(stack);

        return stack_pos - stack;
}

static void
strip_polymer(size_t polymer_length,
              const char *polymer_in,
              char to_strip,
              size_t *polymer_length_out,
              char **polymer_out)
{
        char *out = malloc(polymer_length);
        size_t outpos = 0;

        to_strip = tolower(to_strip);

        for (size_t i = 0; i < polymer_length; i++) {
                if (tolower(polymer_in[i]) != to_strip)
                        out[outpos++] = polymer_in[i];
        }

        *polymer_out = out;
        *polymer_length_out = outpos;
}

static void
read_polymer(FILE *in,
             char **polymer_out,
             size_t *polymer_length)
{
        size_t buf_size = 16;
        size_t buf_length = 0;
        char *buf = malloc(buf_size);

        while (true) {
                buf_length += fread(buf + buf_length,
                                    1,
                                    buf_size - buf_length,
                                    in);

                if (buf_length < buf_size)
                        break;

                buf_size *= 2;
                buf = realloc(buf, buf_size);
        }

        while (buf_length > 0 && isspace(buf[buf_length - 1]))
                buf_length--;

        *polymer_out = buf;
        *polymer_length = buf_length;
}

int
main(int argc, char **argv)
{
        char *polymer;
        size_t polymer_length;

        read_polymer(stdin, &polymer, &polymer_length);

        printf("Part 1: %i\n", (int) reduce_polymer(polymer_length, polymer));

        size_t best_length = SIZE_MAX;

        for (char ch = 'a'; ch <= 'z'; ch++) {
                char *stripped_polymer;
                size_t stripped_polymer_length;

                strip_polymer(polymer_length, polymer,
                              ch,
                              &stripped_polymer_length,
                              &stripped_polymer);

                size_t this_length = reduce_polymer(stripped_polymer_length,
                                                    stripped_polymer);

                free(stripped_polymer);

                if (this_length < best_length)
                        best_length = this_length;
        }

        printf("Part 2: %i\n", (int) best_length);

        free(polymer);

        return EXIT_SUCCESS;
}
