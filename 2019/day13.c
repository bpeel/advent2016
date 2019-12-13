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
        int ball_x, bat_x;
        int64_t score;
};

static bool
input_cb(void *user_data,
         int64_t *value)
{
        struct game *game = user_data;

        if (game->ball_x < game->bat_x)
                *value = -1;
        else if (game->ball_x > game->bat_x)
                *value = 1;
        else
                *value = 0;

        return true;
}

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
                if (game->out_x == -1 && game->out_y == 0)
                        game->score = value;
                else if (value == 3)
                        game->bat_x = game->out_x;
                else if (value == 4)
                        game->ball_x = game->out_x;
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

        for (int part = 1; part <= 2; part++) {
                struct game game = { .grid = grid_new() };
                struct intcode *machine = intcode_new(memory_size, memory);

                intcode_set_input_function(machine, input_cb, &game);
                intcode_set_output_function(machine, output_cb, &game);

                if (part == 2)
                        intcode_write(machine, 0, 2, NULL);

                struct pcx_error *error = NULL;

                if (intcode_run(machine, &error)) {
                        int result = (part == 1 ? count_tiles(game.grid, 2) :
                                      part == 2 ? game.score :
                                      0);

                        printf("Part %i: %i\n", part, result);
                } else {
                        fprintf(stderr, "Part %i: %s\n", part, error->message);
                        pcx_error_free(error);
                        ret = EXIT_FAILURE;
                }

                intcode_free(machine);
                grid_free(game.grid);
        }

        pcx_free(memory);

        return ret;
}
