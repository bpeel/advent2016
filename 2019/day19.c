#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "intcode.h"
#include "read-memory.h"
#include "pcx-error.h"

#define SHIP_SIZE 100

struct query_data {
        int x, y;
        int input_num;
        bool had_result;
        bool result;
};

static struct pcx_error_domain
day19_error;

enum day19_error {
        DAY19_ERROR_NO_RESULT,
        DAY19_ERROR_INVALID_RESULT,
        DAY19_ERROR_FAILED,
};

static bool
input_cb(void *user_data,
         int64_t *value)
{
        struct query_data *data = user_data;

        switch (data->input_num++) {
        case 0:
                *value = data->x;
                break;
        case 1:
                *value = data->y;
                break;
        default:
                return false;
        }

        return true;
}

static bool
output_cb(void *user_data,
          int64_t value)
{
        struct query_data *data = user_data;

        if (data->input_num < 2 || data->had_result)
                return false;

        data->result = value;
        data->had_result = true;

        return true;
}

static bool
query_program(size_t memory_size,
              const int64_t *memory,
              int x, int y,
              bool *result,
              struct pcx_error **error)
{
        struct intcode *machine = intcode_new(memory_size, memory);
        bool ret = true;
        struct query_data data = {
                .x = x, .y = y,
                .input_num = 0,
                .had_result = false,
        };

        intcode_set_input_function(machine, input_cb, &data);
        intcode_set_output_function(machine, output_cb, &data);

        if (!intcode_run(machine, error)) {
                ret = false;
        } else if (!data.had_result) {
                pcx_set_error(error,
                              &day19_error,
                              DAY19_ERROR_NO_RESULT,
                              "Machine didn’t give an output");
                ret = false;
        } else if (data.result != 1 && data.result != 0) {
                pcx_set_error(error,
                              &day19_error,
                              DAY19_ERROR_INVALID_RESULT,
                              "Machine give an invalid output");
                ret = false;
        } else {
                *result = data.result;
        }

        intcode_free(machine);

        return ret;
}

static bool
part1(size_t memory_size,
      const int64_t *memory,
      int *result,
      struct pcx_error **error)
{
        int count = 0;

        for (int y = 0; y < 50; y++) {
                for (int x = 0; x < 50; x++) {
                        bool lit;
                        if (!query_program(memory_size, memory,
                                           x, y,
                                           &lit,
                                           error))
                                return false;
                        if (lit)
                                count++;
                }
        }

        *result = count;

        return true;
}

static bool
part2(size_t memory_size,
      const int64_t *memory,
      int *result,
      struct pcx_error **error)
{
        int y = 0, min_x = 0, max_x = 0;

        while (true) {
                y++;

                for (int i = 0; i < 100; i++) {
                        bool lit;

                        if (!query_program(memory_size,
                                           memory,
                                           min_x + i, y,
                                           &lit,
                                           error))
                                return false;

                        if (lit) {
                                min_x += i;
                                break;
                        }
                }

                if (max_x < min_x)
                        max_x = min_x;

                for (int i = 0; i < 100; i++) {
                        bool lit;

                        if (!query_program(memory_size,
                                           memory,
                                           max_x + i, y,
                                           &lit,
                                           error))
                                return false;

                        if (!lit) {
                                max_x += i - 1;
                                break;
                        }
                }

                for (int x = min_x; x + SHIP_SIZE <= max_x + 1; x++) {
                        bool corner;

                        if (!query_program(memory_size,
                                           memory,
                                           x, y + SHIP_SIZE - 1,
                                           &corner,
                                           error))
                                return false;

                        if (corner) {
                                *result = x * 10000 + y;
                                return true;
                        }
                }
        }
}

int
main(int argc, char **argv)
{
        if (argc != 2) {
                fprintf(stderr, "usage: day19 <program>\n");
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

        int ret = EXIT_SUCCESS;
        int result;

        if (part1(memory_size, memory, &result, &error)) {
                printf("Part 1: %i\n", result);
        } else {
                fprintf(stderr, "Part 1: %s\n", error->message);
                pcx_error_free(error);
                error = NULL;
                ret = EXIT_FAILURE;
        }

        if (part2(memory_size, memory, &result, &error)) {
                printf("Part 2: %i\n", result);
        } else {
                fprintf(stderr, "Part 2: %s\n", error->message);
                pcx_error_free(error);
                error = NULL;
                ret = EXIT_FAILURE;
        }

        pcx_free(memory);

        return ret;
}
