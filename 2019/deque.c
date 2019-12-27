#include "deque.h"

#include <assert.h>
#include <string.h>

#include "pcx-util.h"

void
deque_init(struct deque *deque)
{
        static const struct deque init = DEQUE_STATIC_INIT;

        *deque = init;
}

void
deque_read(struct deque *deque,
           size_t size,
           void *data)
{
        assert(size <= deque->length);

        size_t to_copy = MIN(size, deque->size - deque->start);

        memcpy(data, deque->data + deque->start, to_copy);
        memcpy((uint8_t *) data + to_copy, deque->data, size - to_copy);

        if (size >= deque->length) {
                deque->start = 0;
                deque->length = 0;
        } else {
                deque->start = (deque->start + size) % deque->size;
                deque->length -= size;
        }
}

void
deque_write(struct deque *deque,
            size_t size,
            const void *data)
{
        size_t new_size = MAX(deque->size, 1);

        while (new_size < size)
                new_size *= 2;

        if (new_size != deque->size) {
                uint8_t *buf = pcx_alloc(new_size);
                size_t old_length = deque->length;
                deque_read(deque, old_length, buf);
                deque->length = old_length;
                pcx_free(deque->data);
                deque->data = buf;
                deque->start = 0;
                deque->size = new_size;
        }

        size_t end = (deque->start + deque->length) % deque->size;
        size_t to_copy = MIN(size, deque->size - end);

        memcpy(deque->data + end, data, to_copy);
        memcpy(deque->data, (const uint8_t *) data + to_copy, size - to_copy);

        deque->length += size;
}

void
deque_destroy(struct deque *deque)
{
        pcx_free(deque->data);
}
