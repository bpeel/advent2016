#ifndef INTCODE_H
#define INTCODE_H

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include "pcx-error.h"

struct intcode;

extern struct pcx_error_domain
intcode_error_domain;

enum intcode_error {
        INTCODE_ERROR_INVALID_ADDRESS,
        INTCODE_ERROR_INVALID_OPCODE,
        INTCODE_ERROR_INVALID_ADDRESSING_MODE,
        INTCODE_ERROR_IO,
};

typedef bool
(* intcode_input_function)(void *user_data,
                           int64_t *value);

typedef bool
(* intcode_output_function)(void *user_data,
                            int64_t value);

bool
intcode_read(const struct intcode *machine,
             int64_t address,
             int64_t *value,
             struct pcx_error **error);

bool
intcode_write(struct intcode *machine,
              int64_t address,
              int64_t value,
              struct pcx_error **error);

bool
intcode_step(struct intcode *machine,
             struct pcx_error **error);

bool
intcode_run(struct intcode *machine,
            struct pcx_error **error);

struct intcode *
intcode_new(size_t memory_size,
            const int64_t *memory);

void
intcode_set_input_function(struct intcode *machine,
                           intcode_input_function func,
                           void *user_data);

void
intcode_set_output_function(struct intcode *machine,
                            intcode_output_function func,
                            void *user_data);

void
intcode_free(struct intcode *machine);

#endif /* INTCODE_H */
