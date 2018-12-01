#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

static bool
process_input(FILE *in,
              int *total_score_out,
              int *total_garbage_out)

{
        int depth = 0;
        int total_score = 0;
        int total_garbage = 0;
        enum {
                STATE_NONE,
                STATE_IN_GARBAGE,
                STATE_IN_QUOTE
        } state = STATE_NONE;

        int ch;

        while (true) {
                ch = fgetc(in);

                if (ch == EOF)
                        break;

                switch (state) {
                case STATE_NONE:
                        switch (ch) {
                        case '{':
                                depth++;
                                break;
                        case '}':
                                if (depth < 1) {
                                        fprintf(stderr, "Unbalanced group\n");
                                        return false;
                                }
                                total_score += depth;
                                depth--;
                                break;
                        case ',':
                        case ' ':
                        case '\n':
                        case '\r':
                                break;
                        case '<':
                                state = STATE_IN_GARBAGE;
                                break;
                        default:
                                fprintf(stderr, "Unexpected char: %c\n", ch);
                                return false;
                        }
                        break;
                case STATE_IN_GARBAGE:
                        switch (ch) {
                        case '>':
                                state = STATE_NONE;
                                break;
                        case '!':
                                state = STATE_IN_QUOTE;
                                break;
                        default:
                                total_garbage++;
                                break;
                        }
                        break;
                case STATE_IN_QUOTE:
                        state = STATE_IN_GARBAGE;
                        break;
                }
        }

        if (state != STATE_NONE || depth != 0) {
                fprintf(stderr, "Unexpected EOF\n");
                return false;
        }

        *total_score_out = total_score;
        *total_garbage_out = total_garbage;

        return true;
}

int
main(int argc, char **argv)
{
        int part1, part2;

        if (process_input(stdin, &part1, &part2)) {
                printf("Part 1: %i\n"
                       "Part 2: %i\n",
                       part1,
                       part2);
        }

        return EXIT_SUCCESS;
}
