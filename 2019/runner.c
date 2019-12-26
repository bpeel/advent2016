#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <unistd.h>
#include <errno.h>

#include "intcode.h"
#include "read-memory.h"
#include "pcx-error.h"

static bool
ascii_output_cb(void *user_data,
                int64_t value)
{
        fputc(value, stdout);
        if (value == '\n')
                fflush(stdout);
        return true;
}

static bool
ascii_input_cb(void *user_data,
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
        const char *program_file = NULL;
        bool ascii = false;
        bool override = false;
        int64_t override_value = -1;

        while (true) {
                switch (getopt(argc, argv, "-ao:")) {
                case 1:
                        program_file = optarg;
                        break;
                case 'a':
                        ascii = true;
                        break;
                case 'o': {
                        char *tail;
                        errno = 0;
                        override_value = strtol(optarg, &tail, 10);
                        if (errno || *tail) {
                                fprintf(stderr,
                                        "invalid override value: %s\n",
                                        optarg);
                                return EXIT_FAILURE;
                        }
                        override = true;
                        break;
                }
                case '?':
                        return EXIT_FAILURE;
                case -1:
                        goto finished_args;
                default:
                        fprintf(stderr, "unknown option\n");
                        return EXIT_FAILURE;
                }
        }

finished_args: (void) 0;

        int64_t *memory;
        size_t memory_size;

        struct pcx_error *error = NULL;

        bool read_res;

        if (program_file) {
                read_res = read_memory_from_file(program_file,
                                                 &memory, &memory_size,
                                                 &error);
        } else {
                read_res = read_memory(stdin,
                                       &memory, &memory_size,
                                       &error);
        }

        if (!read_res) {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                return EXIT_FAILURE;
        }

        if (override && memory_size >= sizeof *memory)
                memory[0] = override_value;

        struct intcode *machine = intcode_new(memory_size, memory);

        pcx_free(memory);

        if (ascii) {
                intcode_set_input_function(machine, ascii_input_cb, NULL);
                intcode_set_output_function(machine, ascii_output_cb, NULL);
        }

        int ret = EXIT_SUCCESS;

        if (!intcode_run(machine, &error)) {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                ret = EXIT_FAILURE;
        }

        intcode_free(machine);

        return ret;
}
