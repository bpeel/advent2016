#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "intcode.h"
#include "read-memory.h"
#include "pcx-error.h"

static bool
output_cb(void *user_data,
          int64_t value)
{
        fputc(value, stdout);
        return true;
}

static bool
input_cb(void *user_data,
         int64_t *value)
{
        int ch = fgetc(stdin);

        if (ch == EOF)
                return false;

        *value = ch;

        return true;
}

int
main(int argc, char **argv)
{
        if (argc != 2) {
                fprintf(stderr, "usage: day25 <program>\n");
                return EXIT_FAILURE;
        }

        int64_t *memory;
        size_t memory_size;

        struct pcx_error *error = NULL;

        if (!read_memory_from_file(argv[1], &memory, &memory_size, &error)) {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                return EXIT_FAILURE;
        }

        struct intcode *machine = intcode_new(memory_size, memory);

        pcx_free(memory);

        intcode_set_input_function(machine, input_cb, NULL);
        intcode_set_output_function(machine, output_cb, NULL);

        int ret = EXIT_SUCCESS;

        if (!intcode_run(machine, &error)) {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                ret = EXIT_FAILURE;
        }

        intcode_free(machine);

        return ret;
}
