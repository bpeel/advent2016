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
get_gcd(int a, int b)
{
        while (b != 0) {
                int tmp = b;
                b = a % b;
                a = tmp;
        }

        return a;
}

static int
get_quandrant(const struct asteroid *a)
{
        if (a->dir_x == 0) {
                if (a->dir_y <= 0)
                        return 0;
                else
                        return 2;
        } else if (a->dir_y == 0) {
                if (a->dir_x > 0)
                        return 1;
                else
                        return 3;
        } else if (a->dir_x > 0) {
                if (a->dir_y > 0)
                        return 1;
                else
                        return 0;
        } else {
                if (a->dir_y > 0)
                        return 2;
                else
                        return 3;
        }
}

static int
compare_angle_distance(const void *pa,
                       const void *pb)
{
        const struct asteroid *a = pa;
        const struct asteroid *b = pb;

        if (a->dir_x == b->dir_x && a->dir_y == b->dir_y)
                return a->mult - b->mult;

        if (a->mult == 0)
                return -1;
        if (b->mult == 0)
                return 1;

        int qa = get_quandrant(a);
        int qb = get_quandrant(b);

        if (qa != qb)
                return qa - qb;

        if (a->dir_x == 0 || a->dir_y == 0) {
                if (b->dir_x == 0 || b->dir_y == 0)
                        return 0;
                else
                        return -1;
        } else if (b->dir_x == 0 || b->dir_y == 0) {
                return 1;
        }

        int acx = a->dir_x * b->dir_y;
        int bcx = b->dir_x * a->dir_y;

        return bcx - acx;
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

        /* Sort the asteroids by angle then distance */

        qsort(asteroids,
              n_asteroids,
              sizeof *asteroids,
              compare_angle_distance);

        assert(asteroids[0].mult == 0 &&
               asteroids[0].dir_x == 0 &&
               asteroids[0].dir_y == 0);

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

        for (unsigned i = 2; i < n_asteroids; i++) {
                const struct asteroid *a = asteroids + i - 1;
                const struct asteroid *b = asteroids + i;

                /* If this asteroid is at the same angle as the last
                 * one then it is blocked.
                 */
                if (a->dir_x == b->dir_x &&
                    a->dir_y == b->dir_y)
                        blocked_count++;
        }

        pcx_free(asteroids);

        return n_asteroids - blocked_count - 1;
}

static void
part2(size_t n_asteroids,
      const struct xy_pos *positions,
      const struct xy_pos *base)
{
        struct asteroid *asteroids = get_asteroids(n_asteroids,
                                                   positions,
                                                   base);

        int destroyed_count = 0;
        int pos = 0;

        while (true) {
                /* Skip to the next unblocked asteroid */
                while (asteroids[pos].blocked)
                        pos = (pos + 1) % n_asteroids;

                struct asteroid *a = asteroids + pos;

                a->blocked = true;

                if (++destroyed_count >= n_asteroids)
                        break;

                if (destroyed_count >= 201) {
                        int x = a->dir_x * a->mult + base->x;
                        int y = a->dir_y * a->mult + base->y;

                        printf("Part 2: %i (%i,%i)\n",
                               x * 100 + y,
                               x,
                               y);
                        break;
                }

                /* Skip all of the asteroids that have the same
                 * direction
                 */
                do
                        pos = (pos + 1) % n_asteroids;
                while (asteroids[pos].dir_x == a->dir_x &&
                       asteroids[pos].dir_y == a->dir_y);
        }

        pcx_free(asteroids);
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

        printf("Part 1: %i (%i,%i)\n",
               most_visible,
               positions[best_asteroid].x,
               positions[best_asteroid].y);

        part2(n_asteroids, positions, positions + best_asteroid);

        pcx_free(positions);

        return 0;
}
