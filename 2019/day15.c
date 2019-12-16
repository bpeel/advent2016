#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <assert.h>
#include <limits.h>

#include "intcode.h"
#include "read-memory.h"
#include "grid.h"
#include "pcx-buffer.h"
#include "pcx-error.h"

struct stack_entry {
        int direction_to_try;
};

struct pos_entry {
        int x, y;
        int direction_to_try;
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
print_grid(const struct grid *grid)
{
        struct grid_size size;

        grid_get_size(grid, &size);

        for (int y = 0; y < size.height; y++) {
                for (int x = 0; x < size.width; x++) {
                        int ch;

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
           int direction_to_try)
{
        pcx_buffer_set_length(stack,
                              stack->length + sizeof (struct stack_entry));

        struct stack_entry *entry =
                ((struct stack_entry *) (stack->data + stack->length)) - 1;
        entry->direction_to_try = direction_to_try;
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
backtrack(struct intcode *machine,
          struct build_grid_data *data,
          struct pcx_buffer *stack,
          int *x, int *y,
          struct pcx_error **error)
{
        while (true) {
                stack->length -= sizeof (struct stack_entry);

                if (stack->length <= 0)
                        break;

                struct stack_entry *entry = ((struct stack_entry *)
                                             (stack->data + stack->length)) - 1;

                assert(entry->direction_to_try >= 2);

                int back_dir = ((entry->direction_to_try - 2) ^ 1) + 1;

                int result;

                if (!ask_machine(machine,
                                 data,
                                 back_dir,
                                 &result,
                                 error))
                        return false;

                if (result == 0) {
                        pcx_set_error(error,
                                      &build_grid_error,
                                      BUILD_GRID_ERROR_INCONSISTENT_RESULT,
                                      "Backtracking failed");
                        return false;
                }

                move_robot(back_dir, *x, *y, x, y);

                if (entry->direction_to_try < 5)
                        break;
        }

        return true;
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

        stack_push(&stack, 1);
        grid_write(grid, x, y, 1);

        while (stack.length > 0) {
                int result;
                struct stack_entry *entry = ((struct stack_entry *)
                                             (stack.data + stack.length)) - 1;

                for (;
                     entry->direction_to_try <= 4;
                     entry->direction_to_try++) {
                        int nx, ny;

                        move_robot(entry->direction_to_try, x, y, &nx, &ny);

                        if (grid_read(grid, nx, ny))
                                continue;

                        if (!ask_machine(machine,
                                         &data,
                                         entry->direction_to_try,
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
                        entry->direction_to_try++;
                        stack_push(&stack, 1);
                        goto found_dir;
                }

                if (!backtrack(machine, &data, &stack, &x, &y, error)) {
                        ret = false;
                        goto done;
                }

        found_dir:
                continue;
        }

done:
        pcx_buffer_destroy(&stack);
        intcode_free(machine);

        return ret;
}

static void
pos_push(struct pcx_buffer *stack,
         int x, int y)
{
        pcx_buffer_set_length(stack,
                              stack->length + sizeof (struct pos_entry));

        struct pos_entry *entry =
                ((struct pos_entry *) (stack->data + stack->length)) - 1;

        entry->x = x;
        entry->y = y;
        entry->direction_to_try = 0;
}

static bool
already_visited(const struct pcx_buffer *stack,
                int x, int y)
{
        const struct pos_entry *entries =
                (const struct pos_entry *) stack->data;
        size_t n_entries = stack->length / sizeof *entries;

        for (unsigned i = 0; i < n_entries; i++) {
                if (entries[i].x == x && entries[i].y == y)
                        return true;
        }

        return false;
}

static bool
find_shortest_path(const struct grid *grid,
                   int *result)
{
        struct pcx_buffer stack = PCX_BUFFER_STATIC_INIT;
        int shortest_path = INT_MAX;

        pos_push(&stack, 0, 0);

        while (stack.length > 0) {
                struct pos_entry *entry =
                        ((struct pos_entry *) (stack.data + stack.length)) - 1;

                for (int dir = entry->direction_to_try; dir < 4; dir++) {
                        int x = entry->x;
                        int y = entry->y;

                        switch (dir) {
                        case 0: x--; break;
                        case 1: x++; break;
                        case 2: y--; break;
                        case 3: y++; break;
                        }

                        if (already_visited(&stack, x, y))
                                continue;

                        uint8_t cell = grid_read(grid, x, y);

                        if (cell == 3)
                                continue;

                        if (cell == 2) {
                                int length = stack.length / sizeof *entry;

                                if (length < shortest_path)
                                        shortest_path = length;
                        }

                        entry->direction_to_try = dir + 1;
                        pos_push(&stack, x, y);
                        goto found_dir;
                }

                do {
                        stack.length -= sizeof (struct pos_entry);
                } while (stack.length > 0 &&
                         ((struct pos_entry *) (stack.data + stack.length))[-1].
                         direction_to_try >= 4);

        found_dir:
                continue;
        }

        *result = shortest_path;

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

        int ret = EXIT_SUCCESS;

        for (int part = 1; part <= 1; part++) {
                struct grid *grid = grid_new();
                struct pcx_error *error = NULL;
                int shortest_path;

                if (build_grid(memory_size,
                               memory,
                               grid,
                               &error) &&
                    find_shortest_path(grid, &shortest_path)) {
                        print_grid(grid);
                        printf("Part %i: %i\n", part, shortest_path);
                } else {
                        fprintf(stderr, "Part %i: %s\n", part, error->message);
                        pcx_error_free(error);
                        ret = EXIT_FAILURE;
                }

                grid_free(grid);
        }

        pcx_free(memory);

        return ret;
}
