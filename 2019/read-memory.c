#include "read-memory.h"

#include <ctype.h>

#include "pcx-buffer.h"

bool
read_memory(FILE *in,
            int64_t **buf_out,
            size_t *size_out)
{
        struct pcx_buffer buf = PCX_BUFFER_STATIC_INIT;
        int64_t value = 0;
        bool has_value = true;
        bool negative = false;

        while (true) {
                int ch = fgetc(in);

                if (ch == EOF)
                        break;

                if (ch == '-') {
                        if (has_value || negative)
                                goto error;
                        negative = true;
                } else if (ch >= '0' && ch <= '9') {
                        value = value * 10 + ch - '0';
                        has_value = true;
                } else if (ch == ',') {
                        if (!has_value)
                                goto error;
                        if (negative)
                                value = -value;
                        pcx_buffer_append(&buf, &value, sizeof value);
                        value = 0;
                        has_value = false;
                        negative = false;
                } else if (!isspace(ch)) {
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
