#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
#include <limits.h>
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>

struct disk {
        int16_t next;
        int16_t first_child;
        int16_t parent;
        int weight;
        char *name;
};

struct disk_set {
        size_t buffer_size;
        size_t n_disks;
        struct disk *disks;
};

static int
get_disk(struct disk_set *set,
         const char *name,
         size_t name_length)
{
        for (size_t i = 0; i < set->n_disks; i++) {
                if (strlen(set->disks[i].name) == name_length &&
                    !memcmp(name, set->disks[i].name, name_length))
                        return (int) i;
        }

        if (set->buffer_size <= set->n_disks) {
                if (set->buffer_size == 0) {
                        set->buffer_size = 8;
                        set->disks = malloc(sizeof set->disks[0] *
                                            set->buffer_size);
                } else {
                        set->buffer_size *= 2;
                        set->disks = realloc(set->disks,
                                             sizeof set->disks[0] *
                                             set->buffer_size);
                }
        }

        struct disk *disk = set->disks + set->n_disks;

        disk->name = malloc(name_length + 1);
        memcpy(disk->name, name, name_length);
        disk->name[name_length] = 0;
        disk->first_child = -1;
        disk->next = -1;
        disk->parent = -1;
        disk->weight = INT16_MIN;

        return set->n_disks++;
}

static bool
parse_disk(struct disk_set *set, const char *line)
{
        const char *name_end = strchr(line, ' ');
        int weight;

        if (name_end == NULL)
                return false;

        size_t name_length = name_end - line;

        int disk_num = get_disk(set, line, name_length);

        int res = sscanf(name_end, " (%i)", &weight);

        if (res < 1)
                return false;

        set->disks[disk_num].weight = weight;

        const char *name_start = strstr(name_end, " -> ");

        if (name_start == NULL)
                return true;

        name_start += 4;

        while (true) {
                while (isspace(*name_start) || *name_start == ',')
                        name_start++;

                if (*name_start == '\0')
                        break;

                name_end = strchr(name_start, ',');
                if (name_end == NULL) {
                        name_end = name_start + strlen(name_start);
                        while (name_end > name_start && isspace(name_end[-1]))
                                name_end--;
                }

                int child_num = get_disk(set,
                                         name_start,
                                         name_end - name_start);
                struct disk *child = set->disks + child_num;
                struct disk *parent = set->disks + disk_num;
                child->next = parent->first_child;
                parent->first_child = child_num;
                child->parent = disk_num;

                name_start = name_end;
        }

        return true;
}

static void
free_disk_set(struct disk_set *set)
{
        if (set->disks == NULL)
                return;

        for (size_t i = 0; i < set->n_disks; i++)
                free(set->disks[i].name);

        free(set->disks);
}

static bool
read_disks(struct disk_set *set, FILE *in)
{
        int line_number = 1;
        char line[512];

        while (fgets(line, sizeof line, in)) {
                bool res = parse_disk(set, line);

                if (!res) {
                        fprintf(stderr,
                                "invalid disk at line %i\n",
                                line_number);
                        return false;
                }

                line_number++;
        }

        return true;
}

static int
find_root(const struct disk_set *set)
{
        int root = -1;

        for (size_t i = 0; i < set->n_disks; i++) {
                if (set->disks[i].parent == -1) {
                        if (root == -1) {
                                root = (int) i;
                        } else {
                                fprintf(stderr,
                                        "both %s and %s have no parent\n",
                                        set->disks[root].name,
                                        set->disks[i].name);
                                return -1;
                        }
                }
        }

        return root;
}

static void
show_adjustment(const struct disk_set *set,
                int node,
                int diff)
{
        const struct disk *disk = set->disks + node;

        printf("Part 2: disk %s needs to change from %i to %i\n",
               disk->name,
               disk->weight,
               disk->weight + diff);
}

static int
find_bad_weight(const struct disk_set *set, int node)
{
        int total_weight = set->disks[node].weight;
        int weight_a = -1, weight_b = -1;
        bool have_third_weight = false;
        int weight_a_count = 0, weight_b_count = 0;
        int one_node = -1;

        for (int child = set->disks[node].first_child;
             child != -1;
             child = set->disks[child].next) {
                int child_weight = find_bad_weight(set, child);

                total_weight += child_weight;

                if (weight_a_count == 0) {
                        weight_a = child_weight;
                        weight_a_count = 1;
                        one_node = child;
                } else if (child_weight == weight_a) {
                        weight_a_count++;
                } else if (weight_b_count == 0) {
                        weight_b = child_weight;
                        weight_b_count = 1;
                        one_node = child;
                } else if (child_weight == weight_b) {
                        weight_b_count++;
                } else {
                        have_third_weight = true;
                }
        }

        if (have_third_weight) {
                fprintf(stderr,
                        "More than two weights found for children "
                        "of %s\n",
                        set->disks[node].name);
        } else if (weight_a_count == 1) {
                if (weight_b_count == 1) {
                        fprintf(stderr,
                                "Both children of %s have different weights\n",
                                set->disks[node].name);
                } else if (weight_b_count > 1) {
                        int diff = weight_b - weight_a;
                        show_adjustment(set, one_node, diff);
                        total_weight += diff;
                }
        } else if (weight_b_count == 1) {
                int diff = weight_a - weight_b;
                show_adjustment(set, one_node, weight_a - weight_b);
                total_weight += diff;
        }

        return total_weight;
}

int
main(int argc, char **argv)
{
        struct disk_set set = { 0 };
        int ret = EXIT_SUCCESS;

        if (!read_disks(&set, stdin)) {
                ret = EXIT_FAILURE;
                goto done;
        }

        int root = find_root(&set);

        if (root == -1) {
                ret = EXIT_FAILURE;
                goto done;
        }

        printf("Part 1: %s\n", set.disks[root].name);

        find_bad_weight(&set, root);

done:
        free_disk_set(&set);

        return ret;
}
