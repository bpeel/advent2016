#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <limits.h>

struct bot {
        int coord[3];
        int radius;
};

static int
point_distance(const int *coord_a,
               const int *coord_b)
{
        int sum = 0;

        for (int i = 0; i < 3; i++)
                sum += abs(coord_a[i] - coord_b[i]);

        return sum;
}

static int
bot_contains_point(const struct bot *bot,
                   const int *point)
{
        return point_distance(bot->coord, point) <= bot->radius;
}

static void
read_bots(FILE *in,
          size_t *n_bots_out,
          struct bot **bots_out)
{
        size_t buf_size = 1;
        size_t n_bots = 0;
        struct bot *bots = malloc(sizeof *bots * buf_size);

        while (true) {
                if (n_bots >= buf_size) {
                        buf_size *= 2;
                        bots = realloc(bots, sizeof *bots * buf_size);
                }

                struct bot *bot = bots + n_bots;

                int got = fscanf(in,
                                 "pos=<%i,%i,%i>, r=%i\n",
                                 bot->coord + 0,
                                 bot->coord + 1,
                                 bot->coord + 2,
                                 &bot->radius);
                if (got != 4)
                        break;

                n_bots++;
        }

        *n_bots_out = n_bots;
        *bots_out = bots;
}

static int
max_move(const int *point,
         int axis,
         size_t n_bots,
         const struct bot *bots)
{
        int offset = INT_MAX;

        for (size_t i = 0; i < n_bots; i++) {
                const struct bot *bot = bots + i;

                int max_axis_move =
                        bot->radius -
                        (abs(point[(axis + 1) % 3] -
                             bot->coord[(axis + 1) % 3]) +
                         abs(point[(axis + 2) % 3] -
                             bot->coord[(axis + 2) % 3]));

                int already_offset = bot->coord[axis] - point[axis];
                if (point[axis] < 0)
                        already_offset = -already_offset;

                int bot_offset = max_axis_move - already_offset;

                if (bot_offset < offset)
                        offset = bot_offset;
        }

        return offset;
}

static void
find_best_point(size_t n_bots,
                const struct bot *bots,
                int *point_out)
{
}
