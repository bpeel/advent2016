#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <pthread.h>

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
        bool part2;
};

struct loop_amp {
        struct intcode *machine;
        struct loop_amp *input;
        int64_t output;
        bool has_output;
        pthread_t thread;
        pthread_mutex_t mutex;
        pthread_cond_t cond;
        int phase;
        bool sent_phase;
};

static struct pcx_error_domain
loop_error_domain;

enum loop_error {
        LOOP_ERROR_NO_OUTPUT,
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
loop_input_cb(void *user_data,
              int64_t *value)
{
        struct loop_amp *amp = user_data;

        if (!amp->sent_phase) {
                amp->sent_phase = true;
                *value = amp->phase;
                return true;
        }

        pthread_mutex_lock(&amp->input->mutex);

        while (!amp->input->has_output) {
                pthread_cond_wait(&amp->input->cond,
                                  &amp->input->mutex);
        }

        *value = amp->input->output;
        amp->input->has_output = false;

        pthread_cond_signal(&amp->input->cond);

        pthread_mutex_unlock(&amp->input->mutex);

        return true;
}

static bool
loop_output_cb(void *user_data,
               int64_t value)
{
        struct loop_amp *amp = user_data;

        pthread_mutex_lock(&amp->mutex);

        while (amp->has_output)
                pthread_cond_wait(&amp->cond, &amp->mutex);

        amp->output = value;
        amp->has_output = true;

        pthread_cond_signal(&amp->cond);

        pthread_mutex_unlock(&amp->mutex);

        return true;
}

static void *
sequence_thread_cb(void *user_data)
{
        struct loop_amp *amp = user_data;
        struct pcx_error *error = NULL;

        intcode_run(amp->machine, &error);

        return error;
}

static bool
run_sequence_part2(size_t memory_size,
                   const int64_t *memory,
                   const int *phase_sequence,
                   int64_t *result,
                   struct pcx_error **error)
{
        struct loop_amp amps[N_AMPLIFIERS];
        bool ret = true;

        for (int i = 0; i < N_AMPLIFIERS; i++) {
                amps[i].machine = intcode_new(memory_size, memory);
                amps[i].input = amps + (i + N_AMPLIFIERS - 1) % N_AMPLIFIERS;
                amps[i].has_output = false;
                amps[i].phase = phase_sequence[i];
                amps[i].sent_phase = false;
                pthread_mutex_init(&amps[i].mutex, NULL);
                pthread_cond_init(&amps[i].cond, NULL);

                intcode_set_input_function(amps[i].machine,
                                            loop_input_cb,
                                            amps + i);
                intcode_set_output_function(amps[i].machine,
                                            loop_output_cb,
                                            amps + i);
        }

        amps[N_AMPLIFIERS - 1].has_output = true;
        amps[N_AMPLIFIERS - 1].output = 0;

        for (int i = 0; i < N_AMPLIFIERS; i++) {
                pthread_create(&amps[i].thread,
                               NULL, /* attr */
                               sequence_thread_cb,
                               amps + i);
        }

        for (int i = 0; i < N_AMPLIFIERS; i++) {
                void *retval;

                pthread_join(amps[i].thread, &retval);

                if (retval) {
                        if (ret) {
                                pcx_error_propagate(error, retval);
                                ret = false;
                        } else {
                                pcx_error_free(retval);
                        }
                }
        }

        for (int i = 0; i < N_AMPLIFIERS; i++) {
                pthread_mutex_destroy(&amps[i].mutex);
                pthread_cond_destroy(&amps[i].cond);
                intcode_free(amps[i].machine);
        }

        if (amps[N_AMPLIFIERS - 1].has_output) {
                *result = amps[N_AMPLIFIERS - 1].output;
        } else if (ret) {
                ret = false;
                pcx_set_error(error,
                              &loop_error_domain,
                              LOOP_ERROR_NO_OUTPUT,
                              "Final amp didn’t have any output");
        }

        return ret;
}

static bool
permutation_cb(const int *sequence_in,
               void *user_data)
{
        struct sequence_data *data = user_data;
        int64_t this_result;
        int sequence[N_AMPLIFIERS];

        memcpy(sequence, sequence_in, sizeof sequence);

        if (data->part2) {
                for (int i = 0; i < N_AMPLIFIERS; i++)
                        sequence[i] += N_AMPLIFIERS;

                if (!run_sequence_part2(data->memory_size, data->memory,
                                        sequence,
                                        &this_result,
                                        data->error))
                        return false;
        } else {
                if (!run_sequence(data->memory_size, data->memory,
                                  sequence,
                                  &this_result,
                                  data->error))
                        return false;
        }

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
                  bool part2,
                  int *best_sequence_out,
                  int64_t *best_result_out,
                  struct pcx_error **error)
{
        struct sequence_data data = {
                .best_result = INT64_MIN,
                .best_sequence = { 0 },
                .memory = memory,
                .memory_size = memory_size,
                .error = error,
                .part2 = part2
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

        if (!read_memory(stdin, &memory, &memory_size, NULL)) {
                fprintf(stderr, "Error reading initial memory\n");
                return EXIT_FAILURE;
        }

        struct pcx_error *error = NULL;
        int ret = EXIT_SUCCESS;
        int best_sequence[N_AMPLIFIERS];
        int64_t best_result;

        for (int part = 1; part <= 2; part++) {
                if (get_best_sequence(memory_size, memory,
                                      part == 2,
                                      best_sequence,
                                      &best_result,
                                      &error)) {
                        printf("Part %i: %" PRIi64 " (", part, best_result);
                        for (int i = 0; i < N_AMPLIFIERS; i++)
                                printf(" %i", best_sequence[i]);
                        fputs(" )\n", stdout);
                } else {
                        fprintf(stderr, "%s\n", error->message);
                        pcx_error_free(error);
                        ret = EXIT_FAILURE;
                }
        }

        pcx_free(memory);

        return ret;
}
