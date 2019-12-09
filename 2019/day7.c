#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>

#include "intcode.h"
#include "read-memory.h"
#include "pcx-error.h"
#include "permutations.h"

#define N_AMPLIFIERS 5
#define MAX_PHASE_SETTING 4

struct io_data {
        int64_t input[2];
        size_t input_pos;
        int64_t output;
};

struct sequence_data {
        int64_t best_result;
        int best_sequence[N_AMPLIFIERS];
        const int64_t *memory;
        size_t memory_size;
        struct pcx_error **error;
};

static bool
input_cb(void *user_data,
         int64_t *value)
{
        struct io_data *data = user_data;

        if (data->input_pos >= PCX_N_ELEMENTS(data->input))
                return false;

        *value = data->input[data->input_pos++];

        return true;
}

static bool
output_cb(void *user_data,
          int64_t value)
{
        struct io_data *data = user_data;

        data->output = value;

        return true;
}

static bool
run_machine(size_t memory_size,
            const int64_t *memory,
            int64_t input,
            int64_t phase_setting,
            int64_t *result,
            struct pcx_error **error)
{
        struct intcode *machine = intcode_new(memory_size, memory);

        struct io_data data = {
                .input[0] = phase_setting,
                .input[1] = input,
                .input_pos = 0,
                .output = INT64_MIN
        };

        intcode_set_input_function(machine, input_cb, &data);
        intcode_set_output_function(machine, output_cb, &data);

        bool ret = intcode_run(machine, error);

        intcode_free(machine);

        if (ret)
                *result = data.output;

        return ret;
}

static bool
run_sequence(size_t memory_size,
             const int64_t *memory,
             const int *phase_sequence,
             int64_t *result,
             struct pcx_error **error)
{
        int64_t value = 0;

        for (int i = 0; i < N_AMPLIFIERS; i++) {
                if (!run_machine(memory_size, memory,
                                 value,
                                 phase_sequence[i],
                                 &value,
                                 error))
                        return false;
        }

        *result = value;

        return true;
}

static bool
permutation_cb(const int *sequence,
               void *user_data)
{
        struct sequence_data *data = user_data;
        int64_t this_result;

        if (!run_sequence(data->memory_size, data->memory,
                          sequence,
                          &this_result,
                          data->error))
                return false;

        if (this_result > data->best_result) {
                memcpy(data->best_sequence,
                       sequence,
                       N_AMPLIFIERS * sizeof sequence[0]);
                data->best_result = this_result;
        }

        return true;
}

static bool
get_best_sequence(size_t memory_size,
                  const int64_t *memory,
                  int *best_sequence_out,
                  int64_t *best_result_out,
                  struct pcx_error **error)
{
        struct sequence_data data = {
                .best_result = INT64_MIN,
                .best_sequence = { 0 },
                .memory = memory,
                .memory_size = memory_size,
                .error = error
        };

        if (!permutations(N_AMPLIFIERS, permutation_cb, &data))
                return false;

        memcpy(best_sequence_out,
               data.best_sequence,
               sizeof data.best_sequence);
        *best_result_out = data.best_result;

        return true;
}

int
main(int argc, char **argv)
{
        int64_t *memory;
        size_t memory_size;

        if (!read_memory(stdin, &memory, &memory_size)) {
                fprintf(stderr, "Error reading initial memory\n");
                return EXIT_FAILURE;
        }

        struct pcx_error *error = NULL;
        int ret = EXIT_SUCCESS;
        int best_sequence[N_AMPLIFIERS];
        int64_t best_result;

        if (get_best_sequence(memory_size, memory,
                              best_sequence,
                              &best_result,
                              &error)) {
                printf("Best sequence:");
                for (int i = 0; i < N_AMPLIFIERS; i++)
                        printf(" %i", best_sequence[i]);
                fputc('\n', stdout);
                printf("Best value: %" PRIi64 "\n", best_result);
        } else {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                ret = EXIT_FAILURE;
        }

        pcx_free(memory);

        return ret;
}
