#include <stdint.h>
#include <inttypes.h>
#include <string.h>

#include "intcode.h"
#include "read-memory.h"
#include "grid.h"

struct bot {
        struct grid *grid;
        int painted_count;
        int x, y, dir;
        int input_num;
};

static bool
input_cb(void *user_data,
         int64_t *value)
{
        struct bot *bot = user_data;

        *value = grid_read(bot->grid, bot->x, bot->y) & 0x7f;

        return true;
}

static bool
output_cb(void *user_data,
          int64_t value)
{
        struct bot *bot = user_data;

        if ((bot->input_num & 1) == 0) {
                uint8_t old_value = grid_read(bot->grid, bot->x, bot->y);

                if (old_value == 0)
                        bot->painted_count++;

                grid_write(bot->grid, bot->x, bot->y, value | 0x80);
        } else {
                if (value)
                        bot->dir = (bot->dir + 1) % 4;
                else
                        bot->dir = (bot->dir + 3) % 4;

                switch (bot->dir) {
                case 0:
                        bot->y--;
                        break;
                case 1:
                        bot->x++;
                        break;
                case 2:
                        bot->y++;
                        break;
                case 3:
                        bot->x--;
                        break;
                }
        }

        bot->input_num++;

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

        for (int part = 1; part <= 2; part++) {
                struct bot bot = { .grid = grid_new() };
                struct intcode *machine = intcode_new(memory_size, memory);

                if (part == 2)
                        grid_write(bot.grid, 0, 0, 1);

                intcode_set_input_function(machine, input_cb, &bot);
                intcode_set_output_function(machine, output_cb, &bot);

                struct pcx_error *error = NULL;

                if (intcode_run(machine, &error)) {
                        if (part == 1)
                                printf("Part 1: %i\n", bot.painted_count);
                        else if (!grid_dump(bot.grid, "day11-part2.ppm"))
                                ret = EXIT_FAILURE;
                } else {
                        fprintf(stderr, "%s\n", error->message);
                        pcx_error_free(error);
                        ret = EXIT_FAILURE;
                }

                intcode_free(machine);
                grid_free(bot.grid);
        }

        pcx_free(memory);

        return ret;
}
