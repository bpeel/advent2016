#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <ctype.h>
#include <stdlib.h>
#include <time.h>

struct rule {
        int input_size;
        uint16_t input;
        uint16_t output;
};

struct image {
        size_t size;
        uint16_t patterns[];
};

#define GRID_SIZE(size) (((size) & 1) ? 3 : 2)

struct image *
allocate_image(size_t size)
{
        int grid_size = GRID_SIZE(size);
        int grid_x = size / grid_size;
        int n_patterns = grid_x * grid_x;
        struct image *image = calloc(sizeof *image +
                                     n_patterns * sizeof image->patterns[0],
                                     1);

        image->size = size;

        return image;
}

static uint16_t
rotate_pattern(uint16_t pattern,
               int size)
{
        uint16_t res = 0;

        for (int y = 0; y < size; y++) {
                for (int x = 0; x < size; x++) {
                        if ((pattern & 1)) {
                                int pos = size - 1 - y + x * size;
                                res |= (1 << pos);
                        }
                        pattern >>= 1;
                }
        }

        return res;
}

static uint16_t
flip_pattern_x(uint16_t pattern,
               int size)
{
        uint16_t res = 0;

        for (int y = 0; y < size; y++) {
                for (int x = 0; x < size; x++) {
                        if ((pattern & 1)) {
                                int pos = size - 1 - x + y * size;
                                res |= (1 << pos);
                        }
                        pattern >>= 1;
                }
        }

        return res;
}

static void
print_image(const struct image *image)
{
        int grid_size = GRID_SIZE(image->size);
        int grid_x = image->size / grid_size;
        const uint16_t *patterns = image->patterns;

        for (int y = 0; y < image->size; y++) {
                for (int x = 0; x < grid_x; x++) {
                        uint16_t p = patterns[y / grid_size * grid_x + x];
                        p >>= y % grid_size * grid_size;
                        for (int j = 0; j < grid_size; j++) {
                                fputc((p & 1) ? '#' : '.', stdout);
                                p >>= 1;
                        }
                }
                fputc('\n', stdout);
        }

        fputc('\n', stdout);
}

static bool
parse_pattern(const char **line,
              uint16_t *pattern_out,
              int *size_out)
{
        uint16_t pattern = 0;
        int size = 0, y = 0;
        const char *p = *line;

        while (true) {
                int x = 0;
                while (true) {
                        int ch = *p;
                        if (ch == '#')
                                pattern |= 1 << (x + y * size);
                        else if (ch != '.')
                                break;
                        p++;
                        x++;
                }

                if (y == 0)
                        size = x;
                else if (x != size)
                        return false;

                y++;

                if (*p != '/')
                        break;
                p++;
        }

        if (y != size)
                return false;

        *line = p;
        *pattern_out = pattern;
        *size_out = size;

        return true;
}

static bool
parse_rule(const char *line,
           struct rule *rule)
{
        int input_size, output_size;

        if (!parse_pattern(&line, &rule->input, &input_size))
                return false;

        while (isspace(*line))
                line++;

        if (line[0] != '=' && line[1] != '>')
                return false;
        line += 2;

        while (isspace(*line))
                line++;

        if (!parse_pattern(&line, &rule->output, &output_size))
                return false;

        while (isspace(*line))
                line++;

        if (*line != '\0')
                return false;

        switch (input_size) {
        case 2:
                if (output_size != 3)
                        return false;
                break;
        case 3:
                if (output_size != 4)
                        return false;
                break;
        default:
                return false;
        }

        rule->input_size = input_size;

        return true;
}

static int
compare_rule_input(const struct rule *rule,
                   int input_size,
                   uint16_t pattern)
{
        if (rule->input_size != input_size)
                return input_size - rule->input_size;

        if (rule->input < pattern)
                return -1;
        else if (rule->input > pattern)
                return 1;
        else
                return 0;
}

static int
compare_rule_cb(const void *a,
                const void *b)
{
        const struct rule *rb = b;

        return compare_rule_input(a, rb->input_size, rb->input);
}

static bool
read_rules(FILE *in,
           size_t *n_rules_out,
           struct rule **rules_out)
{
        char line[512];
        size_t buf_size = 8;
        size_t n_rules = 0;
        struct rule *rules = malloc(buf_size * sizeof *rules);
        int line_num = 1;

        while (fgets(line, sizeof line, stdin)) {
                if (n_rules >= buf_size) {
                        buf_size *= 2;
                        rules = realloc(rules, buf_size * sizeof *rules);
                }

                if (!parse_rule(line, rules + n_rules)) {
                        free(rules);
                        fprintf(stderr, "invalid rule on line %i\n", line_num);
                        return false;
                }

                n_rules++;
                line_num++;
        }

        qsort(rules, n_rules, sizeof *rules, compare_rule_cb);

        *n_rules_out = n_rules;
        *rules_out = rules;

        return true;
}

static bool
transform_pattern(size_t n_rules,
                  const struct rule *rules,
                  size_t input_size,
                  uint16_t pattern,
                  uint16_t *pattern_out)
{
        size_t min = 0, max = n_rules;

        while (max > min) {
                size_t mid = (min + max) / 2;
                int comparison = compare_rule_input(rules + mid,
                                                    input_size,
                                                    pattern);
                if (comparison < 0) {
                        min = mid + 1;
                } else if (comparison > 0) {
                        max = mid;
                } else {
                        *pattern_out = rules[mid].output;
                        return true;
                }
        }

        return false;
}

static bool
transform_any_pattern(size_t n_rules,
                      const struct rule *rules,
                      size_t input_size,
                      uint16_t pattern,
                      uint16_t *pattern_out)
{
        for (int i = 0; i < 4; i++) {
                if (transform_pattern(n_rules, rules,
                                      input_size,
                                      pattern, pattern_out))
                        return true;

                uint16_t flipped_pattern = flip_pattern_x(pattern, input_size);

                if (transform_pattern(n_rules, rules,
                                      input_size,
                                      flipped_pattern, pattern_out))
                        return true;

                pattern = rotate_pattern(pattern, input_size);
        }

        return false;
}

static void
set_pattern(struct image *image,
            int off_x,
            int off_y,
            uint16_t pattern)
{
        int grid_size = GRID_SIZE(image->size);
        int grid_x = image->size / grid_size;

        for (int y = 0; y < 3; y++) {
                for (int x = 0; x < 3; x++) {
                        if ((pattern & 1)) {
                                int gx = (off_x + x) / grid_size;
                                int gy = (off_y + y) / grid_size;
                                int p = gx + gy * grid_x;
                                int bx = (off_x + x) % grid_size;
                                int by = (off_y + y) % grid_size;
                                image->patterns[p] |=
                                        1 << (bx + by * grid_size);
                        }

                        pattern >>= 1;
                }
        }
}

static struct image *
transform_image(size_t n_rules,
                const struct rule *rules,
                const struct image *image_in)
{
        int grid_size = GRID_SIZE(image_in->size);
        int grid_x = image_in->size / grid_size;
        int n_patterns = grid_x * grid_x;
        struct image *image_out = allocate_image(grid_x * (grid_size + 1));
        int out_grid_size = GRID_SIZE(image_out->size);

        for (size_t i = 0; i < n_patterns; i++) {
                uint16_t pattern_out;

                if (!transform_any_pattern(n_rules, rules,
                                           grid_size,
                                           image_in->patterns[i],
                                           &pattern_out)) {
                        free(image_out);
                        return false;
                }

                size_t gx = i % grid_x;
                size_t gy = i / grid_x;

                if (grid_size == 2) {
                        if (out_grid_size == 3) {
                                image_out->patterns[i] = pattern_out;
                        } else {
                                set_pattern(image_out,
                                            gx * 3,
                                            gy * 3,
                                            pattern_out);
                        }
                } else {
                        /* Split the pattern into 4 */

                        for (int y = 0; y < 2; y++) {
                                for (int x = 0; x < 2; x++) {
                                        size_t p = (gx * 2 + x +
                                                    (gy * 2 + y) * grid_x * 2);
                                        int shift = x * 2 + y * 8;
                                        image_out->patterns[p] =
                                                (((pattern_out >> shift) & 3) |
                                                 ((pattern_out >> (shift + 2) &
                                                   0xc)));
                                }
                        }
                }
        }

        return image_out;
}

static int
count_pixels(const struct image *image)
{
        int grid_size = GRID_SIZE(image->size);
        int grid_x = image->size / grid_size;
        int n_patterns = grid_x * grid_x;
        int sum = 0;

        for (int i = 0; i < n_patterns; i++)
                sum += __builtin_popcount(image->patterns[i]);

        return sum;
}

static double
time_to_seconds(const struct timespec *ts)
{
        return ts->tv_sec + ts->tv_nsec / 1000000000.0;
}

int
main(int argc, char **argv)
{
        size_t n_rules;
        struct rule *rules;
        int part1 = 0;

        if (!read_rules(stdin, &n_rules, &rules))
                return EXIT_FAILURE;

        struct image *image = allocate_image(3);

        image->patterns[0] = 0x1e2;

        print_image(image);

        int ret = EXIT_SUCCESS;

        struct timespec start_time;
        clock_gettime(CLOCK_MONOTONIC, &start_time);

        for (int i = 0; i < 18; i++) {
                struct image *next_image =
                        transform_image(n_rules, rules, image);

                if (next_image == NULL) {
                        fprintf(stderr, "failed to transform image\n");
                        ret = EXIT_FAILURE;
                        break;
                }

                free(image);
                image = next_image;

                if (i + 1 == 5)
                        part1 = count_pixels(image);
        }

        struct timespec end_time;
        clock_gettime(CLOCK_MONOTONIC, &end_time);

        double elapsed_time = (time_to_seconds(&end_time) -
                               time_to_seconds(&start_time));

        printf("%f seconds\n"
               "Part 1: %i\n"
               "Part 2: %i\n",
               elapsed_time,
               part1,
               count_pixels(image));

        free(rules);
        free(image);

        return ret;
}
