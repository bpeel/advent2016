#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

static int
process_input(FILE *in)
{
        int depth = 0;
        int total_score = 0;
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
                                        return -1;
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
                                return -1;
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
                        }
                        break;
                case STATE_IN_QUOTE:
                        state = STATE_IN_GARBAGE;
                        break;
                }
        }

        if (state != STATE_NONE || depth != 0) {
                fprintf(stderr, "Unexpected EOF\n");
                return -1;
        }

        return total_score;
}

int
main(int argc, char **argv)
{
        int total_score = process_input(stdin);

        if (total_score != -1)
                printf("Part 1: %i\n", total_score);

        return EXIT_SUCCESS;
}
