#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "intcode.h"
#include "read-memory.h"
#include "pcx-error.h"

static bool
run_machine(size_t memory_size,
            const int64_t *memory,
            int64_t noun,
            int64_t verb,
            int64_t *result,
            struct pcx_error **error)
{
        struct intcode *machine = intcode_new(memory_size, memory);

        bool ret = (intcode_write(machine, 1, noun, error) &&
                    intcode_write(machine, 2, verb, error) &&
                    intcode_run(machine, error) &&
                    intcode_read(machine, 0, result, error));

        intcode_free(machine);

        return ret;
}

static void
run_part2(size_t memory_size,
          const int64_t *memory)
{
        for (int64_t noun = 0; noun < 100; noun++) {
                for (int64_t verb = 0; verb < 100; verb++) {
                        struct pcx_error *error = NULL;
                        int64_t result;

                        if (!run_machine(memory_size,
                                         memory,
                                         noun,
                                         verb,
                                         &result,
                                         &error)) {
                                printf("Part 2: %s\n", error->message);
                                pcx_error_free(error);
                                return;
                        }

                        if (result == 19690720) {
                                printf("Part 2: %" PRIi64 "\n",
                                       noun * 100 + verb);
                                return;
                        }
                }
        }
}

int
main(int argc, char **argv)
{
        int ret = EXIT_SUCCESS;
        int64_t *memory;
        size_t memory_size;

        if (!read_memory(stdin, &memory, &memory_size, NULL)) {
                fprintf(stderr, "Error reading initial memory\n");
                return EXIT_FAILURE;
        }

        struct pcx_error *error = NULL;

        int64_t part1;

        if (run_machine(memory_size, memory, 12, 2, &part1, &error)) {
                printf("Part 1: %" PRIi64 "\n", part1);
        } else {
                printf("Part 1: %s\n", error->message);
                pcx_error_free(error);
        }

        run_part2(memory_size, memory);

        pcx_free(memory);

        return ret;
}
