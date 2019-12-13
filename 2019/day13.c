#include <stdint.h>
#include <inttypes.h>
#include <string.h>

#include "intcode.h"
#include "read-memory.h"
#include "grid.h"

struct game {
        struct grid *grid;
        int output_num;
        int out_x, out_y;
};

static bool
output_cb(void *user_data,
          int64_t value)
{
        struct game *game = user_data;

        switch (game->output_num % 3) {
        case 0:
                game->out_x = value;
                break;
        case 1:
                game->out_y = value;
                break;
        case 2:
                grid_write(game->grid, game->out_x, game->out_y, value);
                break;
        }

        game->output_num++;

        return true;
}

static int
count_tiles(const struct grid *grid,
            int tile)
{
        struct grid_size size;
        int count;

        grid_get_size(grid, &size);

        for (int y = 0; y < size.height; y++) {
                for (int x = 0; x < size.width; x++) {
                        if (grid_read(grid,
                                      size.base_x + x,
                                      size.base_y + y) == tile)
                                count++;
                }
        }

        return count;
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

        struct game game = { .grid = grid_new() };
        struct intcode *machine = intcode_new(memory_size, memory);

        intcode_set_output_function(machine, output_cb, &game);

        struct pcx_error *error = NULL;

        if (intcode_run(machine, &error)) {
                printf("Part 1: %i\n", count_tiles(game.grid, 2));
                grid_dump(game.grid, "day13-part1.ppm");
        } else {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                ret = EXIT_FAILURE;
        }

        intcode_free(machine);
        grid_free(game.grid);

        pcx_free(memory);

        return ret;
}
