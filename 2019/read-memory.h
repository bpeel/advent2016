#ifndef READ_MEMORY_H
#define READ_MEMORY_H

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#include "pcx-error.h"

extern struct pcx_error_domain
read_memory_error;

enum read_memory_error {
        READ_MEMORY_ERROR_INVALID,
        READ_MEMORY_ERROR_IO,
};

bool
read_memory(FILE *in,
            int64_t **buf_out,
            size_t *size_out,
            struct pcx_error **error);

#endif /* READ_MEMORY_H */
