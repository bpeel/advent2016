#define _GNU_SOURCE

#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <assert.h>

#include "pcx-buffer.h"
#include "pcx-util.h"

struct asteroid {
        int x, y;
        bool blocked;
};

static void
add_asteroid(struct pcx_buffer *buf,
             int x,
             int y)
{
        pcx_buffer_set_length(buf, buf->length + sizeof (struct asteroid));

        struct asteroid *asteroid =
                (struct asteroid *)
                (buf->data + buf->length - sizeof *asteroid);

        asteroid->x = x;
        asteroid->y = y;
        asteroid->blocked = false;
}

static void
read_asteroids(FILE *in,
               size_t *n_asteroids_out,
               struct asteroid **asteroids_out)
{
        struct pcx_buffer buf = PCX_BUFFER_STATIC_INIT;
        char line[512];
        int y = 0;

        while (fgets(line, sizeof line, in)) {
                for (int x = 0; line[x]; x++) {
                        if (line[x] == '#')
                                add_asteroid(&buf, x, y);
                }

                y++;
        }

        *n_asteroids_out = buf.length / sizeof (struct asteroid);
        *asteroids_out = (struct asteroid *) buf.data;
}

static int
distance(const struct asteroid *a,
         const struct asteroid *b)
{
        return abs(a->x - b->x) + abs(a->y - b->y);
}

static int
compare_distance(const void *a,
                 const void *b,
                 void *base)
{
        return distance(a, base) - distance(b, base);
}

static bool
is_same_side(int base,
             int middle,
             int end)
{
        return (middle < base) == (end < middle);
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
is_blocking(const struct asteroid *base,
            const struct asteroid *middle,
            const struct asteroid *end)
{
        if (middle->x == base->x)
                return end->x == base->x;
        if (middle->y == base->y)
                return end->y == base->y;

        if (!is_same_side(base->x, middle->x, end->x) ||
            !is_same_side(base->y, middle->y, end->y))
                return false;

        int x_dist = abs(middle->x - base->x);
        int y_dist = abs(middle->y - base->y);

        int gcd = get_gcd(x_dist, y_dist);

        x_dist /= gcd;
        y_dist /= gcd;

        int ex_dist = abs(end->x - base->x);
        int ey_dist = abs(end->y - base->y);

        if (ex_dist % x_dist != 0 ||
            ey_dist % y_dist != 0)
                return false;

        return ex_dist / x_dist == ey_dist / y_dist;
}

static int
count_visible(size_t n_asteroids,
              const struct asteroid *asteroids_in,
              const struct asteroid *base)
{
        struct asteroid *asteroids =
                pcx_memdup(asteroids_in, n_asteroids * sizeof *asteroids_in);
        size_t blocked_count = 0;

        /* Sort the asteroids as distance from the base. Further
         * asteroids can not block further ones.
         */

        qsort_r(asteroids,
                n_asteroids,
                sizeof *asteroids,
                compare_distance,
                (void *) base);

        assert(asteroids[0].x == base->x &&
               asteroids[0].y == base->y);

        for (unsigned i = 1; i < n_asteroids; i++) {
                if (asteroids[i].blocked)
                        continue;
                for (unsigned j = i + 1; j < n_asteroids; j++) {
                        if (asteroids[j].blocked)
                                continue;

                        if (is_blocking(asteroids + 0,
                                        asteroids + i,
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
        struct asteroid *asteroids;
        int most_visible = -1;
        int best_asteroid = -1;

        read_asteroids(stdin, &n_asteroids, &asteroids);

        for (unsigned i = 0; i < n_asteroids; i++) {
                int this_count = count_visible(n_asteroids,
                                               asteroids,
                                               asteroids + i);

                if (this_count > most_visible) {
                        most_visible = this_count;
                        best_asteroid = i;
                }
        }

        pcx_free(asteroids);

        printf("Part 1: %i (%i,%i)\n",
               most_visible,
               asteroids[best_asteroid].x,
               asteroids[best_asteroid].y);

        return 0;
}
