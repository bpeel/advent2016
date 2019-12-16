#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <assert.h>
#include <limits.h>
#include <unistd.h>

#include "intcode.h"
#include "read-memory.h"
#include "grid.h"
#include "pcx-buffer.h"
#include "pcx-error.h"

struct stack_entry {
        int direction_taken;
};

struct build_grid_data {
        bool took_input;
        bool has_result;
        int64_t result;
        int64_t input;
};

static struct pcx_error_domain
build_grid_error;

enum {
        BUILD_GRID_ERROR_INVALID_RESULT,
        BUILD_GRID_ERROR_INCONSISTENT_RESULT,
};

static void
print_grid(const struct grid *grid, int rx, int ry)
{
        struct grid_size size;

        grid_get_size(grid, &size);

        for (int y = 0; y < size.height; y++) {
                for (int x = 0; x < size.width; x++) {
                        int ch;

                        if (rx == x + size.base_x && ry == y + size.base_y) {
                                fputc('r', stdout);
                                continue;
                        }

                        switch (grid_read(grid,
                                          x + size.base_x,
                                          y + size.base_y)) {
                        case 0: ch = '?'; break;
                        case 1: ch = '.'; break;
                        case 2: ch = 'o'; break;
                        case 3: ch = '#'; break;
                        default: assert(!"unexpected grid value");
                        }

                        fputc(ch, stdout);
                }

                fputc('\n', stdout);
        }
}

static bool
build_grid_input_cb(void *user_data,
                    int64_t *value)
{
        struct build_grid_data *data = user_data;

        if (data->took_input)
                return false;

        data->took_input = true;
        *value = data->input;

        return true;
}

static bool
build_grid_output_cb(void *user_data,
                     int64_t value)
{
        struct build_grid_data *data = user_data;

        if (!data->took_input)
                return false;

        data->result = value;
        data->has_result = true;

        return true;
}

static bool
ask_machine(struct intcode *machine,
            struct build_grid_data *data,
            int input,
            int *result,
            struct pcx_error **error)
{
        data->took_input = false;
        data->has_result = false;
        data->input = input;

        do {
                if (!intcode_step(machine, error))
                        return false;
        } while (!data->has_result);

        if (data->result < 0 || data->result > 2) {
                pcx_set_error(error,
                              &build_grid_error,
                              BUILD_GRID_ERROR_INVALID_RESULT,
                              "Invalid result %" PRIi64 " from intcode machine",
                              data->result);
                return false;
        }

        *result = data->result;

        return true;
}

static void
stack_push(struct pcx_buffer *stack,
           int direction_taken)
{
        pcx_buffer_set_length(stack,
                              stack->length + sizeof (struct stack_entry));

        struct stack_entry *entry =
                ((struct stack_entry *) (stack->data + stack->length)) - 1;
        entry->direction_taken = direction_taken;
}

static void
move_robot(int dir,
           int x, int y,
           int *nx, int *ny)
{
        switch (dir) {
        case 1: *nx = x; *ny = y - 1; break;
        case 2: *nx = x; *ny = y + 1; break;
        case 3: *nx = x - 1; *ny = y; break;
        case 4: *nx = x + 1; *ny = y; break;
        default: assert(false);
        };
}

static bool
build_grid(size_t memory_size,
           const int64_t *memory,
           struct grid *grid,
           struct pcx_error **error)
{
        struct intcode *machine = intcode_new(memory_size, memory);
        struct build_grid_data data;

        intcode_set_input_function(machine, build_grid_input_cb, &data);
        intcode_set_output_function(machine, build_grid_output_cb, &data);

        struct pcx_buffer stack = PCX_BUFFER_STATIC_INIT;
        int x = 0, y = 0;
        bool ret = true;

        stack_push(&stack, 0);
        grid_write(grid, x, y, 1);

        while (stack.length > 0) {
                fputs("\033[2J", stdout);
                print_grid(grid, x, y);
                fputs("\n\n", stdout);
                sleep(1);

                int result;
                struct stack_entry entry = ((struct stack_entry *)
                                            (stack.data + stack.length))[-1];
                stack.length -= sizeof (struct stack_entry);

                for (int dir = entry.direction_taken + 1; dir <= 4; dir++) {
                        int nx, ny;

                        move_robot(dir, x, y, &nx, &ny);

                        if (grid_read(grid, nx, ny))
                                continue;

                        if (!ask_machine(machine,
                                         &data,
                                         dir,
                                         &result,
                                         error)) {
                                ret = false;
                                goto done;
                        }

                        if (result == 0) {
                                grid_write(grid, nx, ny, 3);
                                continue;
                        }

                        grid_write(grid, nx, ny, result);
                        x = nx;
                        y = ny;
                        stack_push(&stack, dir);
                        stack_push(&stack, 0);
                        goto found_dir;
                }

                if (entry.direction_taken == 0)
                        continue;

                /* Backtrack */
                int back_dir = ((entry.direction_taken - 1) ^ 1) + 1;

                if (!ask_machine(machine,
                                 &data,
                                 back_dir,
                                 &result,
                                 error)) {
                        ret = false;
                        goto done;
                }

                if (result == 0) {
                        pcx_set_error(error,
                                      &build_grid_error,
                                      BUILD_GRID_ERROR_INCONSISTENT_RESULT,
                                      "Backtracking failed");
                        ret = false;
                        goto done;
                }

                move_robot(back_dir, x, y, &x, &y);

        found_dir:
                continue;
        }

done:
        pcx_buffer_destroy(&stack);
        intcode_free(machine);

        return ret;
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

        int ret = EXIT_SUCCESS;

        for (int part = 1; part <= 1; part++) {
                struct grid *grid = grid_new();
                struct pcx_error *error = NULL;

                if (!build_grid(memory_size,
                                memory,
                                grid,
                                &error)) {
                        fprintf(stderr, "Part %i: %s\n", part, error->message);
                        pcx_error_free(error);
                        ret = EXIT_FAILURE;
                }

                print_grid(grid, INT_MIN, INT_MIN);

                grid_free(grid);
        }

        pcx_free(memory);

        return ret;
}
