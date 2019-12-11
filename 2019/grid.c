#include "grid.h"

#include <stdio.h>
#include <string.h>
#include <errno.h>

#include "pcx-util.h"

struct grid {
        int base_x, base_y;
        int width, height;
        uint8_t *data;
};

static bool
grid_contains_x(const struct grid *grid, int x)
{
        return x >= grid->base_x && x < grid->base_x + grid->width;
}

static bool
grid_contains_y(const struct grid *grid, int y)
{
        return y >= grid->base_y && y < grid->base_y + grid->height;
}

static bool
grid_contains_point(const struct grid *grid,
                    int x, int y)
{
        return grid_contains_x(grid, x) && grid_contains_y(grid, y);
}

static size_t
grid_get_offset(const struct grid *grid,
                int x, int y)
{
        return (x - grid->base_x) + (y - grid->base_y) * grid->width;
}

uint8_t
grid_read(const struct grid *grid,
          int x, int y)
{
        if (grid_contains_point(grid, x, y))
                return grid->data[grid_get_offset(grid, x, y)];
        else
                return 0;
}

static void
grid_resize_for_x(struct grid *grid,
                  int x)
{
        if (grid_contains_x(grid, x))
                return;

        int new_width = grid->width;
        int needed_width;

        if (x < grid->base_x)
                needed_width = grid->width + grid->base_x - x;
        else
                needed_width = x - grid->base_x + 1;

        do
                new_width *= 2;
        while (new_width < needed_width);

        uint8_t *new_data = pcx_alloc(new_width * grid->height);
        uint8_t *dst = new_data;
        const uint8_t *src = grid->data;

        for (int y = 0; y < grid->height; y++) {
                if (x < grid->base_x) {
                        memset(dst, 0, new_width - grid->width);
                        memcpy(dst + new_width - grid->width,
                               src,
                               grid->width);
                } else {
                        memcpy(dst, src, grid->width);
                        memset(dst + grid->width, 0, new_width - grid->width);
                }

                dst += new_width;
                src += grid->width;
        }

        if (x < grid->base_x)
                grid->base_x -= new_width - grid->width;

        grid->width = new_width;

        pcx_free(grid->data);
        grid->data = new_data;
}

static void
grid_resize_for_y(struct grid *grid,
                  int y)
{
        if (grid_contains_y(grid, y))
                return;

        int new_height = grid->height;
        int needed_height;

        if (y < grid->base_y)
                needed_height = grid->height + grid->base_y - y;
        else
                needed_height = y - grid->base_y + 1;

        do
                new_height *= 2;
        while (new_height < needed_height);

        uint8_t *new_data = pcx_alloc(new_height * grid->width);

        if (y < grid->base_y) {
                memcpy(new_data + (new_height - grid->height) * grid->width,
                       grid->data,
                       grid->height * grid->width);
                memset(new_data, 0, (new_height - grid->height) * grid->width);
        } else {
                memcpy(new_data, grid->data, grid->height * grid->width);
                memset(new_data + grid->height * grid->width,
                       0,
                       (new_height - grid->height) * grid->width);
        }

        if (y < grid->base_y)
                grid->base_y -= new_height - grid->height;

        grid->height = new_height;

        pcx_free(grid->data);
        grid->data = new_data;
}

void
grid_write(struct grid *grid,
           int x, int y,
           uint8_t value)
{
        grid_resize_for_x(grid, x);
        grid_resize_for_y(grid, y);

        grid->data[grid_get_offset(grid, x, y)] = value;
}

struct grid *
grid_new(void)
{
        struct grid *grid = pcx_calloc(sizeof *grid);
        grid->data = pcx_calloc(1);
        grid->width = 1;
        grid->height = 1;
        return grid;
}

void
grid_free(struct grid *grid)
{
        pcx_free(grid->data);
        pcx_free(grid);
}

bool
grid_dump(const struct grid *grid,
          const char *filename)
{
        FILE *out = fopen(filename, "wb");

        if (out == NULL) {
                fprintf(stderr, "%s: %s\n", filename, strerror(errno));
                return false;
        }

        fprintf(out,
                "P6\n"
                "%i %i\n"
                "255\n",
                grid->width, grid->height);

        const uint8_t *p = grid->data;

        for (int y = 0; y < grid->height; y++) {
                for (int x = 0; x < grid->width; x++) {
                        if (*(p++) & 0x7f)
                                fwrite("\xff\xff\xff", 1, 3, out);
                        else
                                fwrite("\x00\x00\x00", 1, 3, out);
                }
        }

        fclose(out);

        return true;
}
