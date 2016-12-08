#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

#define WIDTH 50
#define HEIGHT 6
#define PIXEL_SIZE 8
#define FRAMES_PER_STEP 1
#define ROW_SCALE 2

static void
write_pixel(FILE *out,
            int pixel)
{
        static const uint8_t on[] = { 255, 0, 0 };
        static const uint8_t off[] = { 0, 0, 0 };

        fwrite(pixel ? on : off, 1, sizeof on, out);
}

static void
write_row_line(FILE *out,
               const uint8_t *row,
               int length)
{
        int i, j;

        for (i = 0; i < length; i++) {
                for (j = 0; j < PIXEL_SIZE; j++)
                        write_pixel(out, row[i]);
        }
}

static void
write_rotated_row(FILE *out,
                  const uint8_t *pixels,
                  int row,
                  int amount)
{
        uint8_t first_pixel = pixels[row * WIDTH];
        uint8_t last_pixel = pixels[HEIGHT - 1 + row * WIDTH];
        int y, x;

        for (y = 0; y < PIXEL_SIZE * ROW_SCALE; y++) {
                for (x = 0; x < amount; x++)
                        write_pixel(out, last_pixel);
                write_row_line(out, pixels + row * WIDTH, WIDTH - 1);
                for (x = 0; x < PIXEL_SIZE - amount; x++)
                        write_pixel(out, first_pixel);
        }
}

static void
write_row(FILE *out,
          const uint8_t *pixels,
          int row)
{
        int y;

        for (y = 0; y < PIXEL_SIZE * ROW_SCALE; y++)
                write_row_line(out, pixels + row * WIDTH, WIDTH);
}

static void
write_rotated_row_frame(FILE *out,
                        const uint8_t *pixels,
                        int row,
                        int amount)
{
        int y;

        for (y = 0; y < row; y++)
                write_row(out, pixels, y);

        write_rotated_row(out, pixels, row, amount);

        for (y = row + 1; y < HEIGHT; y++)
                write_row(out, pixels, y);
}

static void
rotate_row(FILE *out,
           uint8_t *pixels,
           int row)
{
        uint8_t tmp;
        int i, j;

        for (i = 1; i < PIXEL_SIZE; i++) {
                for (j = 0; j < FRAMES_PER_STEP; j++)
                        write_rotated_row_frame(out, pixels, row, i);
        }

        tmp = pixels[row * WIDTH + WIDTH - 1];
        memmove(pixels + row * WIDTH + 1, pixels + row * WIDTH, WIDTH - 1);
        pixels[row * WIDTH] = tmp;

        write_rotated_row_frame(out, pixels, row, 0);
}

static void
write_rotated_column_row(FILE *out,
                         const uint8_t *pixels,
                         int column,
                         int row,
                         int column_pixel)
{
        int i;

        write_row_line(out, pixels + row * WIDTH, column);

        for (i = 0; i < PIXEL_SIZE; i++)
                write_pixel(out, column_pixel);

        write_row_line(out,
                       pixels + column + 1 + row * WIDTH,
                       WIDTH - column - 1);
}

static void
write_rotated_column_frame(FILE *out,
                           const uint8_t *pixels,
                           int column,
                           int amount)
{
        int prev_pixel, this_pixel;
        int column_pixel;
        int y, p, r;

        for (y = 0; y < HEIGHT; y++) {
                prev_pixel = pixels[column + (y + HEIGHT - 1) % HEIGHT * WIDTH];
                this_pixel = pixels[column + y * WIDTH];

                for (p = 0; p < PIXEL_SIZE; p++) {
                        if (p < amount)
                                column_pixel = prev_pixel;
                        else
                                column_pixel = this_pixel;
                        for (r = 0; r < ROW_SCALE; r++) {
                                write_rotated_column_row(out,
                                                         pixels,
                                                         column,
                                                         y,
                                                         column_pixel);
                        }
                }
        }
}

static void
rotate_column(FILE *out,
              uint8_t *pixels,
              int column)
{
        uint8_t tmp;
        int i, j;

        for (i = 1; i < PIXEL_SIZE; i++) {
                for (j = 0; j < FRAMES_PER_STEP; j++)
                        write_rotated_column_frame(out, pixels, column, i);
        }

        tmp = pixels[column + (HEIGHT - 1) * WIDTH];

        for (i = HEIGHT - 2; i >= 0; i--)
                pixels[column + (i + 1) * WIDTH] = pixels[column + i * WIDTH];

        pixels[column] = tmp;

        write_rotated_column_frame(out, pixels, column, 0);
}

static void
write_normal_frame(FILE *out,
                   const uint8_t *pixels)
{
        int y;

        for (y = 0; y < HEIGHT; y++)
                write_row(out, pixels, y);
}

static void
set_rectangle(FILE *out,
              uint8_t *pixels,
              int w,
              int h)
{
        int y, x, f;

        for (y = 0; y < h; y++) {
                for (x = 0; x < w; x++) {
                        pixels[x + y * WIDTH] = 1;
                        for (f = 0; f < FRAMES_PER_STEP; f++)
                                write_normal_frame(out, pixels);
                }
        }
}

int
main(int argc, char **argv)
{
        uint8_t pixels[WIDTH * HEIGHT] = { 0 };
        char line[128];
        int len;
        int x, y, w, h, by, i;

        if (isatty(STDOUT_FILENO)) {
                fprintf(stderr, "not writing video to terminal\n");
                exit(EXIT_FAILURE);
        }

        while (fgets(line, sizeof line, stdin)) {
                len = strlen(line);
                if (len > 0 && line[len - 1] == '\n')
                        line[--len] = '\0';

                if (sscanf(line, "rect %ix%i", &w, &h) == 2) {
                        set_rectangle(stdout, pixels, w, h);
                } else if (sscanf(line,
                                  "rotate row y=%i by %i",
                                  &y, &by) == 2) {
                        for (i = 0; i < by; i++)
                                rotate_row(stdout, pixels, y);
                } else if (sscanf(line,
                                  "rotate column x=%i by %i",
                                  &x, &by) == 2) {
                        for (i = 0; i < by; i++)
                                rotate_column(stdout, pixels, x);
                } else {
                        fprintf(stderr, "invalid line: %s\n", line);
                        exit(EXIT_FAILURE);
                }
        }

        for (i = 0; i < 60; i++)
                write_normal_frame(stdout, pixels);

        return 0;
}
