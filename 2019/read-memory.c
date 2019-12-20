#include "read-memory.h"

#include <ctype.h>
#include <errno.h>
#include <string.h>

#include "pcx-buffer.h"

struct pcx_error_domain
read_memory_error;

bool
read_memory(FILE *in,
            int64_t **buf_out,
            size_t *size_out,
            struct pcx_error **error)
{
        struct pcx_buffer buf = PCX_BUFFER_STATIC_INIT;
        int64_t value = 0;
        bool has_value = true;
        bool negative = false;

        while (true) {
                errno = 0;
                int ch = fgetc(in);

                if (ch == EOF) {
                        if (errno) {
                                pcx_set_error(error,
                                              &read_memory_error,
                                              READ_MEMORY_ERROR_IO,
                                              "Error reading from file: %s",
                                              strerror(errno));
                                goto error;
                        }
                        break;
                }

                if (ch == '-') {
                        if (has_value || negative) {
                                pcx_set_error(error,
                                              &read_memory_error,
                                              READ_MEMORY_ERROR_INVALID,
                                              "Unexpected ‘-’ in input");
                                goto error;
                        }
                        negative = true;
                } else if (ch >= '0' && ch <= '9') {
                        value = value * 10 + ch - '0';
                        has_value = true;
                } else if (ch == ',') {
                        if (!has_value) {
                                pcx_set_error(error,
                                              &read_memory_error,
                                              READ_MEMORY_ERROR_INVALID,
                                              "Empty value in input");
                                goto error;
                        }
                        if (negative)
                                value = -value;
                        pcx_buffer_append(&buf, &value, sizeof value);
                        value = 0;
                        has_value = false;
                        negative = false;
                } else if (!isspace(ch)) {
                        pcx_set_error(error,
                                      &read_memory_error,
                                      READ_MEMORY_ERROR_INVALID,
                                      "Invalid character in input");
                        goto error;
                }
        }

        if (has_value)
                pcx_buffer_append(&buf, &value, sizeof value);

        *buf_out = (int64_t *) buf.data;
        *size_out = buf.length / sizeof (int64_t);

        return true;

error:
        pcx_buffer_destroy(&buf);
        return false;
}
