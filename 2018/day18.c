#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

enum acre_state {
        ACRE_STATE_OPEN,
        ACRE_STATE_TREES,
        ACRE_STATE_LUMBERYARD
};

struct area {
        int size;
        enum acre_state state[];
};

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

static struct area *
new_area(int size)
{
        struct area *area = malloc(sizeof (struct area) +
                                   size * size * sizeof area->state[0]);
        area->size = size;
        return area;
}

static struct area *
copy_area(const struct area *area_in)
{
        struct area *area_out = new_area(area_in->size);
        memcpy(area_out, area_in,
               sizeof (struct area) +
               area_in->size * area_in->size * sizeof area_in->state[0]);
        return area_out;
}

static struct area *
read_area(FILE *in)
{
        struct area *area = NULL;
        char line[512];

        for (int y = 0; area == NULL || y < area->size; y++) {
                if (fgets(line, sizeof line, in) == NULL) {
                        fprintf(stderr, "unexpected EOF on line %i\n", y + 1);
                        goto error;
                }

                if (area == NULL) {
                        int area_size = strlen(line) - 1;
                        if (area_size < 1) {
                                fprintf(stderr, "first line empty\n");
                                goto error;
                        }
                        area = new_area(area_size);
                } else if (strlen(line) - 1 != area->size||
                           line[area->size] != '\n') {
                        fprintf(stderr, "short line on line %i\n", y + 1);
                        goto error;
                }

                for (int x = 0; x < area->size; x++) {
                        enum acre_state state;

                        switch(line[x]) {
                        case '.':
                                state = ACRE_STATE_OPEN;
                                break;
                        case '#':
                                state = ACRE_STATE_LUMBERYARD;
                                break;
                        case '|':
                                state = ACRE_STATE_TREES;
                                break;
                        default:
                                fprintf(stderr,
                                        "unexpected character '%c' "
                                        "on line %i\n",
                                        line[x],
                                        y + 1);
                                goto error;
                        }

                        area->state[y * area->size + x] = state;
                }
        }

        if (fgetc(in) != EOF) {
                fprintf(stderr, "garbage at end of input\n");
                goto error;
        }

        return area;

error:
        if (area)
                free(area);
        return NULL;
}

static int
count_neighbours(const struct area *area,
                 int x_center, int y_center,
                 enum acre_state state)
{
        int min_x = MAX(0, x_center - 1), min_y = MAX(0, y_center - 1);
        int max_x = MIN(area->size, x_center + 2);
        int max_y = MIN(area->size, y_center + 2);
        int count = 0;

        const enum acre_state *pin = area->state + min_y * area->size;

        for (int y = min_y; y < max_y; y++) {
                for (int x = min_x; x < max_x; x++) {
                        if (x == x_center && y == y_center)
                                continue;
                        if (pin[x] == state)
                                count++;
                }

                pin += area->size;
        }

        return count;
}

static int
count_all(const struct area *area,
          enum acre_state state)
{
        int count = 0;

        for (int i = 0; i < area->size * area->size; i++) {
                if (area->state[i] == state)
                        count++;
        }

        return count;
}

static void
step_area(const struct area *area_in,
          struct area *area_out)
{
        const enum acre_state *pin = area_in->state;
        enum acre_state *pout = area_out->state;

        assert(area_in->size == area_out->size);

        for (int y = 0; y < area_in->size; y++) {
                for (int x = 0; x < area_in->size; x++) {
                        switch (*pin) {
                        case ACRE_STATE_OPEN:
                                if (count_neighbours(area_in,
                                                     x, y,
                                                     ACRE_STATE_TREES) >= 3) {
                                        *pout = ACRE_STATE_TREES;
                                } else {
                                        *pout = ACRE_STATE_OPEN;
                                }
                                break;
                        case ACRE_STATE_TREES: {
                                int lumberyards =
                                        count_neighbours(area_in,
                                                         x, y,
                                                         ACRE_STATE_LUMBERYARD);
                                if (lumberyards >= 3)
                                        *pout = ACRE_STATE_LUMBERYARD;
                                else
                                        *pout = ACRE_STATE_TREES;
                                break;
                        }
                        case ACRE_STATE_LUMBERYARD: {
                                int lumberyards =
                                        count_neighbours(area_in,
                                                         x, y,
                                                         ACRE_STATE_LUMBERYARD);
                                int trees =
                                        count_neighbours(area_in,
                                                         x, y,
                                                         ACRE_STATE_TREES);
                                if (lumberyards >= 1 && trees >= 1)
                                        *pout = ACRE_STATE_LUMBERYARD;
                                else
                                        *pout = ACRE_STATE_OPEN;
                                break;
                        }
                        }

                        pin++;
                        pout++;
                }
        }
}

static int
run_steps(const struct area *initial_state,
          unsigned long n_steps)
{
        struct area *area_a = copy_area(initial_state);
        struct area *area_b = new_area(area_a->size);

        for (unsigned long minute = 0; minute < n_steps; minute++) {
                step_area(area_a, area_b);

                struct area *t = area_a;
                area_a = area_b;
                area_b = t;
        }

        int trees = count_all(area_a, ACRE_STATE_TREES);
        int lumberyards = count_all(area_a, ACRE_STATE_LUMBERYARD);

        free(area_b);
        free(area_a);

        return trees * lumberyards;
}

int
main(int argc, char **argv)
{
        struct area *area = read_area(stdin);

        if (area == NULL)
                return EXIT_FAILURE;

        printf("Part 1: %i\n", run_steps(area, 10));
        printf("Part 2: %i\n", run_steps(area, 1000000000ul));

        free(area);

        return EXIT_SUCCESS;
}
