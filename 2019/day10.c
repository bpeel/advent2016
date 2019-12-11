#define _GNU_SOURCE

#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <assert.h>
#include <string.h>

#include "pcx-buffer.h"
#include "pcx-util.h"

struct xy_pos {
        int x, y;
};

struct asteroid {
        bool blocked;
        int dir_x, dir_y, mult;
};

static void
add_position(struct pcx_buffer *buf,
             int x,
             int y)
{
        pcx_buffer_set_length(buf, buf->length + sizeof (struct xy_pos));

        struct xy_pos *xy_pos =
                (struct xy_pos *)
                (buf->data + buf->length - sizeof *xy_pos);

        xy_pos->x = x;
        xy_pos->y = y;
}

static void
read_positions(FILE *in,
               size_t *n_asteroids_out,
               struct xy_pos **asteroids_out)
{
        struct pcx_buffer buf = PCX_BUFFER_STATIC_INIT;
        char line[512];
        int y = 0;

        while (fgets(line, sizeof line, in)) {
                for (int x = 0; line[x]; x++) {
                        if (line[x] == '#')
                                add_position(&buf, x, y);
                }

                y++;
        }

        *n_asteroids_out = buf.length / sizeof (struct xy_pos);
        *asteroids_out = (struct xy_pos *) buf.data;
}

static int
compare_distance(const void *pa,
                 const void *pb)
{
        const struct asteroid *a = pa;
        const struct asteroid *b = pb;
        int ax = a->dir_x * a->mult;
        int ay = a->dir_y * a->mult;
        int a_dist2 = ax * ax + ay * ay;
        int bx = b->dir_x * b->mult;
        int by = b->dir_y * b->mult;
        int b_dist2 = bx * bx + by * by;

        return a_dist2 - b_dist2;
}

static int
get_gcd(int a, int b)
{
        while (b != 0) {
                int tmp = b;
                b = a % b;
                a = tmp;
        }

        return a;
}

static bool
is_blocking(const struct asteroid *middle,
            const struct asteroid *end)
{
        return (middle->dir_x == end->dir_x &&
                middle->dir_y == end->dir_y &&
                middle->mult < end->mult);
}

static struct asteroid *
get_asteroids(size_t n_asteroids,
              const struct xy_pos *positions,
              const struct xy_pos *base)
{
        struct asteroid *asteroids =
                pcx_alloc(n_asteroids * sizeof *asteroids);

        for (unsigned i = 0; i < n_asteroids; i++) {
                int x = positions[i].x - base->x;
                int y = positions[i].y - base->y;

                if (x == 0) {
                        asteroids[i].dir_x = 0;
                        asteroids[i].dir_y = (y == 0 ? 0 :
                                              y < 0 ? -1 :
                                              1);
                        asteroids[i].mult = abs(y);
                } else if (y == 0) {
                        asteroids[i].dir_x = (x < 0 ? -1 :
                                              1);
                        asteroids[i].dir_y = 0;
                        asteroids[i].mult = abs(x);
                } else {
                        int gcd = get_gcd(abs(x), abs(y));

                        asteroids[i].dir_x = x / gcd;
                        asteroids[i].dir_y = y / gcd;
                        asteroids[i].mult = gcd;
                }

                asteroids[i].blocked = false;
        }

        return asteroids;
}

static int
count_visible(size_t n_asteroids,
              const struct xy_pos *positions,
              const struct xy_pos *base)
{
        struct asteroid *asteroids = get_asteroids(n_asteroids,
                                                   positions,
                                                   base);
        size_t blocked_count = 0;

        /* Sort the asteroids as distance from the base. Further
         * asteroids can not block further ones.
         */

        qsort(asteroids,
              n_asteroids,
              sizeof *asteroids,
              compare_distance);

        assert(asteroids[0].mult == 0 &&
               asteroids[0].dir_x == 0 &&
               asteroids[0].dir_y == 0);

        for (unsigned i = 1; i < n_asteroids; i++) {
                if (asteroids[i].blocked)
                        continue;
                for (unsigned j = i + 1; j < n_asteroids; j++) {
                        if (asteroids[j].blocked)
                                continue;

                        if (is_blocking(asteroids + i,
                                        asteroids + j)) {
                                asteroids[j].blocked = true;
                                blocked_count++;
                        }
                }
        }

        pcx_free(asteroids);

        return n_asteroids - blocked_count - 1;
}

int
main(int argc, char **argv)
{
        size_t n_asteroids;
        struct xy_pos *positions;
        int most_visible = -1;
        int best_asteroid = -1;

        read_positions(stdin, &n_asteroids, &positions);

        for (unsigned i = 0; i < n_asteroids; i++) {
                int this_count = count_visible(n_asteroids,
                                               positions,
                                               positions + i);

                if (this_count > most_visible) {
                        most_visible = this_count;
                        best_asteroid = i;
                }
        }

        pcx_free(positions);

        printf("Part 1: %i (%i,%i)\n",
               most_visible,
               positions[best_asteroid].x,
               positions[best_asteroid].y);


        return 0;
}
