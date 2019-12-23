#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>
#include <ctype.h>
#include <limits.h>
#include <stdlib.h>
#include <assert.h>

#include "pcx-util.h"
#include "pcx-buffer.h"
#include "pcx-error.h"

struct map_square {
        uint16_t wall : 1;
        uint16_t teleport_direction : 3;
        uint16_t has_teleport : 4;
        uint16_t teleport_pos;
};

struct map {
        unsigned width, height;
        struct map_square *squares;
        int start_x, start_y;
        int end_x, end_y;
};

struct teleport {
        char label[3];
        int direction;
        int x, y;
};

static struct pcx_error_domain
map_error;

enum map_error {
        MAP_ERROR_BAD_LABEL,
};

enum direction {
        DIRECTION_UP,
        DIRECTION_DOWN,
        DIRECTION_LEFT,
        DIRECTION_RIGHT,
};

struct pos_entry {
        int x, y;
        enum direction direction_to_try;
};

static void
free_map(struct map *map)
{
        pcx_free(map->squares);
        pcx_free(map);
}

static void
read_file_grid(FILE *in,
               char **buf_out,
               int *width_out,
               int *height_out)
{
        struct pcx_buffer line_buf = PCX_BUFFER_STATIC_INIT;
        int width = 0, height = 0;

        while (true) {
                int line_length = 0;

                while (true) {
                        pcx_buffer_ensure_size(&line_buf,
                                               line_buf.length + 32);
                        char *chunk_start =
                                (char *) line_buf.data + line_buf.length;
                        bool eof = !fgets(chunk_start,
                                          line_buf.size - line_buf.length,
                                          in);

                        if (eof) {
                                if (line_length == 0)
                                        goto done;
                                break;
                        }

                        int chunk_length = strlen(chunk_start);

                        line_length += chunk_length;
                        line_buf.length += chunk_length;

                        if (chunk_length > 0 &&
                            chunk_start[chunk_length - 1] == '\n')
                                break;
                }

                while (line_length > 0 &&
                       isspace(line_buf.data[line_buf.length - 1])) {
                        line_length--;
                        line_buf.length--;
                }

                if (line_length > width)
                        width = line_length;

                pcx_buffer_append_c(&line_buf, '\0');

                height++;
        }

done: (void) 0;

        char *grid = pcx_alloc(width * height * sizeof *grid);
        const char *src = (char *) line_buf.data;
        char *dst = grid;

        for (int y = 0; y < height; y++) {
                size_t line_length = strlen(src);
                memcpy(dst, src, line_length);
                memset(dst + line_length, ' ', width - line_length);
                src += line_length + 1;
                dst += width;
        }

        *buf_out = grid;
        *width_out = width;
        *height_out = height;

        pcx_buffer_destroy(&line_buf);
}

static void
bad_label(const char *label,
          struct pcx_error **error)
{
        pcx_set_error(error,
                      &map_error,
                      MAP_ERROR_BAD_LABEL,
                      "Label %s has no nearby space",
                      label);
}

static void
add_teleport(struct pcx_buffer *buf,
             const char *label,
             int x, int y,
             int direction)
{
        pcx_buffer_set_length(buf, buf->length + sizeof (struct teleport));

        struct teleport *teleport =
                ((struct teleport *) (buf->data + buf->length)) - 1;

        teleport->x = x;
        teleport->y = y;
        teleport->direction = direction;
        strcpy(teleport->label, label);
}

static int
compare_teleport_label(const void *ptr_a,
                       const void *ptr_b)
{
        const struct teleport *a = ptr_a;
        const struct teleport *b = ptr_b;

        return strcmp(a->label, b->label);
}

static void
move_direction(enum direction dir,
               int *x, int *y)
{
        switch (dir) {
        case DIRECTION_LEFT: (*x)--; break;
        case DIRECTION_RIGHT: (*x)++; break;
        case DIRECTION_UP: (*y)--; break;
        case DIRECTION_DOWN: (*y)++; break;
        }
}

static void
set_teleport(struct map *map,
             const struct teleport *src,
             const struct teleport *dst)
{
        struct map_square *src_square =
                map->squares + src->x + src->y * map->width;
        src_square->has_teleport = true;
        src_square->teleport_direction = src->direction;
        src_square->teleport_pos = dst->x + dst->y * map->width;
}

static void
block_teleport(struct map *map,
               const struct teleport *teleport)
{
        int x = teleport->x, y = teleport->y;
        move_direction(teleport->direction, &x, &y);
        assert(x >= 0 && x < map->width);
        assert(y >= 0 && y < map->height);
        map->squares[x + y * map->width].wall = true;
}

static bool
link_teleports(struct map *map,
               size_t n_teleports,
               const struct teleport *teleports,
               struct pcx_error **error)
{
        bool found_start = false, found_end = false;

        for (unsigned i = 0; i < n_teleports; i++) {
                if (!strcmp(teleports[i].label, "AA")) {
                        if (found_start) {
                                pcx_set_error(error,
                                              &map_error,
                                              MAP_ERROR_BAD_LABEL,
                                              "Map has more than one start");
                                return false;
                        }

                        map->start_x = teleports[i].x;
                        map->start_y = teleports[i].y;

                        block_teleport(map, teleports + i);

                        found_start = true;

                        continue;
                }

                if (!strcmp(teleports[i].label, "ZZ")) {
                        if (found_end) {
                                pcx_set_error(error,
                                              &map_error,
                                              MAP_ERROR_BAD_LABEL,
                                              "Map has more than one end");
                                return false;
                        }

                        map->end_x = teleports[i].x;
                        map->end_y = teleports[i].y;

                        block_teleport(map, teleports + i);

                        found_end = true;

                        continue;
                }

                if (i + 1 >= n_teleports ||
                    strcmp(teleports[i].label,
                           teleports[i + 1].label)) {
                        pcx_set_error(error,
                                      &map_error,
                                      MAP_ERROR_BAD_LABEL,
                                      "Label %s has no pair",
                                      teleports[i].label);
                        return false;
                }

                set_teleport(map, teleports + i, teleports + i + 1);
                set_teleport(map, teleports + i + 1, teleports + i);
                i++;
        }

        if (!found_start) {
                pcx_set_error(error,
                              &map_error,
                              MAP_ERROR_BAD_LABEL,
                              "Map has no start");
                return false;
        }

        if (!found_end) {
                pcx_set_error(error,
                              &map_error,
                              MAP_ERROR_BAD_LABEL,
                              "Map has no end");
                return false;
        }

        return true;
}

static struct map *
read_map(FILE *in,
         struct pcx_error **error)
{
        int grid_width, grid_height;
        char *grid;

        read_file_grid(in, &grid, &grid_width, &grid_height);

        struct map *map = pcx_alloc(sizeof *map);

        map->width = grid_width;
        map->height = grid_height;
        map->squares = pcx_alloc(grid_width * grid_height *
                                 sizeof *map->squares);

        struct map_square *square = map->squares;
        const char *src = grid;
        struct pcx_buffer teleports = PCX_BUFFER_STATIC_INIT;

        for (int y = 0; y < grid_height; y++) {
                for (int x = 0; x < grid_width; x++) {
                        square->wall = *src == '#';
                        square->has_teleport = false;

                        if (isalpha(*src)) {
                                char label[3];
                                int tx, ty, direction;

                                label[0] = *src;
                                label[2] = '\0';

                                if (y + 1 < grid_height &&
                                    isalpha(src[grid_width])) {
                                        label[1] = src[grid_width];

                                        if (y + 2 < grid_height &&
                                            src[grid_width * 2] == '.') {
                                                tx = x;
                                                ty = y + 2;
                                                direction = DIRECTION_UP;
                                        } else if (y > 0 &&
                                                   src[-grid_width] == '.') {
                                                tx = x;
                                                ty = y - 1;
                                                direction = DIRECTION_DOWN;
                                        } else {
                                                bad_label(label, error);
                                                goto error;
                                        }

                                        add_teleport(&teleports,
                                                     label,
                                                     tx, ty,
                                                     direction);
                                } else if (x + 1 < grid_width &&
                                           isalpha(src[1])) {
                                        label[1] = src[1];

                                        if (x + 2 < grid_width &&
                                            src[2] == '.') {
                                                tx = x + 2;
                                                ty = y;
                                                direction = DIRECTION_LEFT;
                                        } else if (x > 0 && src[-1] == '.') {
                                                tx = x - 1;
                                                ty = y;
                                                direction = DIRECTION_RIGHT;
                                        } else {
                                                bad_label(label, error);
                                                goto error;
                                        }

                                        add_teleport(&teleports,
                                                     label,
                                                     tx, ty,
                                                     direction);
                                }

                        }

                        src++;
                        square++;
                }
        }

        size_t n_teleports = teleports.length / sizeof (struct teleport);

        qsort(teleports.data,
              n_teleports,
              sizeof (struct teleport),
              compare_teleport_label);

        if (!link_teleports(map,
                            n_teleports,
                            (const struct teleport *) teleports.data,
                            error))
                goto error;

        pcx_buffer_destroy(&teleports);
        pcx_free(grid);

        return map;

error:
        pcx_buffer_destroy(&teleports);
        pcx_free(grid);
        free_map(map);

        return NULL;
}

static void
pos_push(struct pcx_buffer *stack,
         int x, int y)
{
        pcx_buffer_set_length(stack,
                              stack->length + sizeof (struct pos_entry));

        struct pos_entry *entry =
                ((struct pos_entry *) (stack->data + stack->length)) - 1;

        entry->x = x;
        entry->y = y;
        entry->direction_to_try = 0;
}

static int
get_best_visited(const struct map *map,
                 const struct pcx_buffer *best_visited,
                 int x,
                 int y)
{
        int grid_pos = x + y * map->width;

        if (grid_pos * sizeof (int) >= best_visited->length)
                return INT_MAX;

        return ((int *) best_visited->data)[grid_pos];
}

static void
set_best_visited(const struct map *map,
                 struct pcx_buffer *best_visited,
                 int x,
                 int y,
                 int value)
{
        int grid_pos = x + y * map->width;

        if (grid_pos * sizeof (int) >= best_visited->length) {
                pcx_buffer_ensure_size(best_visited,
                                       (grid_pos + 1) * sizeof (int));
                for (unsigned i = best_visited->length / sizeof (int);
                     i <= grid_pos;
                     i++) {
                        ((int *) best_visited->data)[i] = INT_MAX;
                }

                best_visited->length = (grid_pos + 1) * sizeof (int);
        }

        ((int *) best_visited->data)[grid_pos] = value;
}

static bool
find_next_direction(const struct map *map,
                    struct pcx_buffer *best_visited,
                    struct pcx_buffer *stack)
{
        struct pos_entry *entry =
                ((struct pos_entry *) (stack->data + stack->length)) - 1;
        int depth = stack->length / sizeof (struct pos_entry);

        for (enum direction dir = entry->direction_to_try; dir < 4; dir++) {
                int x = entry->x;
                int y = entry->y;

                const struct map_square *square =
                        map->squares + x + y * map->width;

                if (square->has_teleport &&
                    square->teleport_direction == dir) {
                        x = square->teleport_pos % map->width;
                        y = square->teleport_pos / map->width;
                } else {
                        move_direction(dir, &x, &y);
                }

                if (x < 0 || x >= map->width ||
                    y < 0 || y >= map->height ||
                    map->squares[x + y * map->width].wall ||
                    get_best_visited(map, best_visited, x, y) < depth)
                        continue;

                set_best_visited(map, best_visited, x, y, depth);
                entry->direction_to_try = dir + 1;
                pos_push(stack, x, y);
                return true;
        }

        return false;

}

static void
find_path(const struct map *map,
          int part,
          int *result)
{
        struct pcx_buffer stack = PCX_BUFFER_STATIC_INIT;
        struct pcx_buffer best_visited = PCX_BUFFER_STATIC_INIT;

        pos_push(&stack, map->start_x, map->start_y);

        while (stack.length > 0) {
                if (find_next_direction(map, &best_visited, &stack))
                        continue;

                do {
                        stack.length -= sizeof (struct pos_entry);
                } while (stack.length > 0 &&
                         ((struct pos_entry *)
                          (stack.data + stack.length))[-1].
                         direction_to_try >= 4);
        }

        *result = get_best_visited(map, &best_visited, map->end_x, map->end_y);

        pcx_buffer_destroy(&best_visited);
        pcx_buffer_destroy(&stack);
}

int
main(int argc, char **argv)
{
        struct pcx_error *error = NULL;
        struct map *map = read_map(stdin, &error);

        if (map == NULL) {
                fprintf(stderr, "%s\n", error->message);
                pcx_error_free(error);
                return EXIT_FAILURE;
        }

        int ret = EXIT_SUCCESS;

        for (int part = 1; part <= 2; part++) {
                int path;

                find_path(map, part, &path);
                printf("Part %i: %i\n", part, path);
        }

        free_map(map);

        return ret;
}
