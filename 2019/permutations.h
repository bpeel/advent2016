#ifndef PERMUTATIONS_H
#define PERMUTATIONS_H

#include <stdbool.h>
#include <stdlib.h>

typedef bool
(* permutations_cb)(const int *sequence,
                    void *user_data);

bool
permutations(size_t n_values,
             permutations_cb cb,
             void *user_data);

#endif /* PERMUTATIONS_H */
