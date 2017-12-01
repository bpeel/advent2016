#include <stdlib.h>
#include <stdio.h>

int
main(int argc, char **argv)
{
        int first_digit = -1;
        int last_digit = -1;
        int next_digit;
        int ch;
        int sum = 0;

        while ((ch = fgetc(stdin)) != EOF) {
                if (ch < '0' || ch > '9')
                        continue;

                next_digit = ch - '0';

                if (first_digit == -1) {
                        first_digit = next_digit;
                } else if (next_digit == last_digit) {
                        sum += next_digit;
                }

                last_digit = next_digit;
        }

        if (last_digit == first_digit)
                sum += last_digit;

        printf("sum = %i\n", sum);

        return EXIT_SUCCESS;
}
