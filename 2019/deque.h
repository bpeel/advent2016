#ifndef DEQUE_H
#define DEQUE_H

#include <stdint.h>
#include <stdarg.h>

#include "pcx-util.h"

struct deque {
        uint8_t *data;
        size_t length;
        size_t size;
        size_t start;
};

#define DEQUE_STATIC_INIT { .data = NULL, .length = 0, .size = 0, .start = 0 }

void
deque_init(struct deque *deque);

void
deque_read(struct deque *deque,
           size_t n_bytes,
           void *data);

void
deque_write(struct deque *deque,
            size_t n_bytes,
            const void *data);

void
deque_destroy(struct deque *buffer);

#endif /* DEQUE_H */
