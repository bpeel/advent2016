#ifndef READ_MEMORY_H
#define READ_MEMORY_H

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

bool
read_memory(FILE *in,
            int64_t **buf_out,
            size_t *size_out);

#endif /* READ_MEMORY_H */
