#include "permutations.h"

static void
swap(int *a, int *b)
{
        int tmp = *a;
        *a = *b;
        *b = tmp;
}

bool
permutations(size_t n_values,
             permutations_cb cb,
             void *user_data)
{
        int *sequence = malloc(n_values * sizeof *sequence);
        int *stack = malloc((n_values + 1) * sizeof *stack);
        int depth = 0;
        bool ret = true;

        for (int i = 0; i < n_values; i++)
                sequence[i] = i;

        stack[0] = -1;

        while (true) {
                if (depth >= n_values) {
                        if (!cb(sequence, user_data)) {
                                ret = false;
                                break;
                        }
                }

                if (++stack[depth] >= n_values) {
                        depth--;
                        if (depth < 0)
                                break;
                        swap(sequence + stack[depth], sequence + depth);
                } else {
                        swap(sequence + stack[depth], sequence + depth);
                        depth++;
                        stack[depth] = depth - 1;
                }
        }

        free(sequence);
        free(stack);

        return ret;
}
