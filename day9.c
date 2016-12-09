#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdbool.h>
#include <ctype.h>

#define BUF_SIZE 512

static ssize_t
strip_spaces(char *buf, ssize_t len)
{
        char *out = buf, *in;

        for (in = buf; in < buf + len; in++) {
                if (!isspace(*in))
                        *(out++) = *in;
        }

        return out - buf;
}

static ssize_t
parse_number(const char *buf, ssize_t len, int *result)
{
        const char *p = buf;
        int value = 0;

        while (len > 0 && *p >= '0' && *p <= '9') {
                value = value * 10 + *p - '0';
                p++;
                len--;
        }

        *result = value;

        return p - buf;
}

static ssize_t
process_bracket(const char *buf, ssize_t len)
{
        const char *p = buf, *end = buf + len;
        int repeat_count, str_len, i;

        if (end <= p || *p != '(')
                return 0;
        p++;

        p += parse_number(p, end - p, &str_len);

        if (end <= p || *p != 'x')
                return 0;
        p++;

        p += parse_number(p, end - p, &repeat_count);

        if (end <= p || *p != ')')
                return 0;
        p++;

        if (end - p < str_len)
                return 0;

        for (i = 0; i < str_len; i++)
                fwrite(p, 1, str_len, stdout);

        return p - buf + str_len;
}

int
main(int argc, char **argv)
{
        int buf_len = 0;
        char buf[BUF_SIZE];
        const char *p, *bracket;
        ssize_t got;

        while (true) {
                got = read(STDIN_FILENO, buf + buf_len, BUF_SIZE - buf_len);

                if (got <= 0)
                        break;

                got = strip_spaces(buf + buf_len, got);

                buf_len += got;

                p = buf;

                while (true) {
                        bracket = memchr(p, '(', buf + buf_len - p);

                        if (bracket == NULL) {
                                fwrite(p, 1, buf + buf_len - p, stdout);
                                p = buf + buf_len;
                                break;
                        }

                        fwrite(p, 1, bracket - p, stdout);
                        p = bracket;

                        got = process_bracket(p, buf + buf_len - p);
                        if (got <= 0)
                                break;

                        p += got;
                }

                memmove(buf, p, buf + buf_len - p);
                buf_len = buf + buf_len - p;
        }

        return 0;
}
