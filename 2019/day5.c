#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <errno.h>

#include "intcode.h"
#include "read-memory.h"
#include "pcx-error.h"

static bool
load_program(const char *filename,
             int64_t **memory_out,
             size_t *size_out)
{
        FILE *f = fopen(filename, "r");

        if (f == NULL) {
                fprintf(stderr,
                        "%s: %s\n",
                        filename,
                        strerror(errno));
                return false;
        }

        bool ret = read_memory(f, memory_out, size_out, NULL);

        fclose(f);

        if (!ret) {
                fprintf(stderr,
                        "%s: invalid program\n",
                        filename);
                return false;
        }

        return true;
}

int
main(int argc, char **argv)
{
        if (argc != 2) {
                fprintf(stderr, "usage: day5 <program>\n");
                return EXIT_FAILURE;
        }

        int64_t *memory;
        size_t memory_size;

        if (!load_program(argv[1], &memory, &memory_size))
                return EXIT_FAILURE;

        struct intcode *machine = intcode_new(memory_size, memory);

        pcx_free(memory);

        struct pcx_error *error = NULL;
        int ret = EXIT_SUCCESS;

        if (!intcode_run(machine, &error)) {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                ret = EXIT_FAILURE;
        }

        intcode_free(machine);

        return ret;
}
