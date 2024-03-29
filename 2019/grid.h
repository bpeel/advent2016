#ifndef GRID_H
#define GRID_H

#include <stdint.h>
#include <stdbool.h>

struct grid;

struct grid_size {
        int base_x, base_y;
        int width, height;
};

uint8_t
grid_read(const struct grid *grid,
          int x, int y);

void
grid_write(struct grid *grid,
           int x, int y,
           uint8_t value);

void
grid_get_size(const struct grid *grid,
              struct grid_size *size);

struct grid *
grid_new(void);

void
grid_free(struct grid *grid);

bool
grid_dump(const struct grid *grid,
          const char *filename);

#endif /* GRID_H */
