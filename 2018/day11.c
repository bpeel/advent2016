#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define GRID_SIZE 300

static int
power_level(int x, int y, int serial_number)
{
        int rack_id = x + 11;
        int base_power_level = rack_id * (y + 1);
        int increase_power_level = base_power_level + serial_number;
        int rack_power_level = increase_power_level * rack_id;
        int hundreds = rack_power_level / 100 % 10;
        return hundreds - 5;
}

int
main(int argc, char **argv)
{
        if (argc != 2) {
                fprintf(stderr, "usage: %s <serial_id>", argv[0]);
                return EXIT_SUCCESS;
        }

        int serial_number = strtol(argv[1], NULL, 10);

        int best_sum = INT_MIN;
        int best_x = 0;
        int best_y = 0;
        int best_square_size = 0;

        for (int x = 0; x < GRID_SIZE; x++) {
                for (int y = 0; y < GRID_SIZE; y++) {
                        int sum = 0;
                        int max_coord = x > y ? x : y;

                        for (int square_size = 0;
                             square_size < GRID_SIZE - max_coord;
                             square_size++) {
                                for (int xx = 0; xx < square_size; xx++) {
                                        sum += power_level(x + xx,
                                                           y + square_size - 1,
                                                           serial_number);
                                }
                                for (int yy = 0; yy < square_size - 1; yy++) {
                                        sum += power_level(x + square_size - 1,
                                                           y + yy,
                                                           serial_number);
                                }
                                if (sum > best_sum) {
                                        best_sum = sum;
                                        best_x = x;
                                        best_y = y;
                                        best_square_size = square_size;
                                }
                        }
                }
        }

        printf("Part 2: %i,%i,%i\n", best_x + 1, best_y + 1, best_square_size);

        return EXIT_SUCCESS;
}
