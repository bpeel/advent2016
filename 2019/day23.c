#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <pthread.h>

#include "intcode.h"
#include "read-memory.h"
#include "pcx-error.h"
#include "deque.h"

#define N_COMPUTERS 50

struct computer {
        struct deque input_queue;
        struct intcode *machine;
        pthread_t thread;
        pthread_mutex_t mutex;
        int64_t output[3];
        int output_pos;
        struct network *network;
};

struct network {
        struct computer computers[N_COMPUTERS];
};

static bool
input_cb(void *user_data,
         int64_t *value)
{
        struct computer *data = user_data;

        pthread_mutex_lock(&data->mutex);

        if (data->input_queue.length >= sizeof *value)
                deque_read(&data->input_queue, sizeof *value, value);
        else
                *value = -1;

        pthread_mutex_unlock(&data->mutex);

        return true;
}

static bool
output_cb(void *user_data,
          int64_t value)
{
        struct computer *data = user_data;

        data->output[data->output_pos++] = value;

        if (data->output_pos >= 3) {
                if (data->output[0] == 255)
                        printf("Part 1: %" PRIi64 "\n", data->output[2]);

                if (data->output[0] >= 0 && data->output[0] < N_COMPUTERS) {
                        struct computer *other =
                                data->network->computers + data->output[0];
                        pthread_mutex_lock(&other->mutex);
                        deque_write(&other->input_queue,
                                    sizeof (int64_t) * 2,
                                    data->output + 1);
                        pthread_mutex_unlock(&other->mutex);
                }
                data->output_pos = 0;
        }

        return true;
}

static void *
thread_cb(void *user_data)
{
        struct computer *computer = user_data;
        struct pcx_error *error = NULL;

        intcode_run(computer->machine, &error);

        return error;
}

static bool
run_network(size_t memory_size,
            const int64_t *memory,
            struct pcx_error **error)
{
        struct network network;
        bool ret = true;

        for (int i = 0; i < N_COMPUTERS; i++) {
                struct computer *comp = network.computers + i;

                comp->machine = intcode_new(memory_size, memory);
                deque_init(&comp->input_queue);
                const int64_t addr = i;
                deque_write(&comp->input_queue, sizeof addr, &addr);
                pthread_mutex_init(&comp->mutex, NULL);
                comp->network = &network;
                comp->output_pos = 0;

                intcode_set_input_function(comp->machine,
                                           input_cb,
                                           comp);
                intcode_set_output_function(comp->machine,
                                            output_cb,
                                            comp);
        }

        for (int i = 0; i < N_COMPUTERS; i++) {
                pthread_create(&network.computers[i].thread,
                               NULL, /* attr */
                               thread_cb,
                               network.computers + i);
        }

        for (int i = 0; i < N_COMPUTERS; i++) {
                void *retval;

                pthread_join(network.computers[i].thread, &retval);

                if (retval) {
                        if (ret) {
                                pcx_error_propagate(error, retval);
                                ret = false;
                        } else {
                                pcx_error_free(retval);
                        }
                }
        }

        for (int i = 0; i < N_COMPUTERS; i++) {
                struct computer *comp = network.computers + i;

                pthread_mutex_destroy(&comp->mutex);
                deque_destroy(&comp->input_queue);
                intcode_free(comp->machine);
        }

        return ret;
}

int
main(int argc, char **argv)
{
        int64_t *memory;
        size_t memory_size;

        if (argc != 2) {
                fprintf(stderr, "usage: day23 <program>\n");
                return EXIT_FAILURE;
        }

        if (!read_memory_from_file(argv[1], &memory, &memory_size, NULL)) {
                fprintf(stderr, "Error reading initial memory\n");
                return EXIT_FAILURE;
        }

        struct pcx_error *error = NULL;
        int ret = EXIT_SUCCESS;

        if (!run_network(memory_size, memory, &error)) {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                ret = EXIT_FAILURE;
        }

        pcx_free(memory);

        return ret;
}
