#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
#include <limits.h>
#include <assert.h>

#include "fv-list.h"

struct disk {
        struct fv_list link;
        struct fv_list children;
        int weight;
        char name[1];
};

static void
free_disk(struct disk *disk);

static struct disk *
create_named_disk(const char *name,
                  size_t name_length)
{
        struct disk *disk = fv_alloc(sizeof *disk + name_length);
        memcpy(disk->name, name, name_length);
        disk->name[name_length] = '\0';
        fv_list_init(&disk->children);
        disk->weight = INT_MIN;

        return disk;
}

static struct disk *
parse_disk(const char *line)
{
        const char *name_end = strchr(line, ' ');
        struct disk *disk;

        if (name_end == NULL)
                return NULL;

        size_t name_length = name_end - line;

        disk = create_named_disk(line, name_length);

        int res = sscanf(name_end, " (%i)", &disk->weight);

        if (res < 1) {
                fv_free(disk);
                return NULL;
        }

        const char *name_start = strstr(name_end, " -> ");

        if (name_start == NULL)
                return disk;

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

                struct disk *child =
                        create_named_disk(name_start, name_end - name_start);
                fv_list_insert(&disk->children, &child->link);
                name_start = name_end;
        }

        return disk;
}

static void
free_disk_list(struct fv_list *list)
{
        struct disk *disk, *tmp;

        fv_list_for_each_safe(disk, tmp, list, link)
                free_disk(disk);
}

static void
free_disk(struct disk *disk)
{
        free_disk_list(&disk->children);
        fv_free(disk);
}

static bool
find_parent(struct fv_list *list,
            struct disk *disk,
            struct disk **parent_out,
            struct disk **stub_out)
{
        struct disk *parent, *stub;

        fv_list_for_each(parent, list, link) {
                fv_list_for_each(stub, &parent->children, link) {
                        if (!strcmp(stub->name, disk->name)) {
                                *parent_out = parent;
                                *stub_out = stub;
                                return true;
                        }
                }
        }

        return false;
}

static bool
build_tree(struct fv_list *list,
           struct disk **root_out)
{
        struct disk *disk, *tmp, *parent, *stub, *root = NULL;

        fv_list_for_each_safe(disk, tmp, list, link) {
                fv_list_remove(&disk->link);

                if (!find_parent(list, disk, &parent, &stub)) {
                        if (root) {
                                fprintf(stderr,
                                        "no parent for %s, but %s is already "
                                        "the root\n",
                                        disk->name,
                                        root->name);
                                free_disk(disk);
                                free_disk(root);
                                return false;
                        } else {
                                root = disk;
                                continue;
                        }
                }

                if (stub->weight != INT_MIN) {
                        fprintf(stderr,
                                "duplicate node found for %s\n",
                                disk->name);
                        free_disk(disk);
                        if (root)
                                free_disk(root);
                        return false;
                }

                fv_list_remove(&stub->link);
                free_disk(stub);
                fv_list_insert(parent->children.prev, &disk->link);
        }

        assert(fv_list_empty(list));

        if (root == NULL) {
                fprintf(stderr, "no root found\n");
                return false;
        }

        *root_out = root;

        return true;
}

static bool
read_disks(struct fv_list *disks, FILE *in)
{
        int line_number = 1;
        char line[512];
        struct disk *disk;

        while (fgets(line, sizeof line, in)) {
                disk = parse_disk(line);

                if (disk == NULL) {
                        fprintf(stderr,
                                "invalid disk at line %i\n",
                                line_number);
                        return false;
                }

                fv_list_insert(disks, &disk->link);

                line_number++;
        }

        return true;
}

int
main(int argc, char **argv)
{
        struct fv_list disks;
        struct disk *root;
        int ret = EXIT_SUCCESS;

        fv_list_init(&disks);

        if (read_disks(&disks, stdin) &&
            build_tree(&disks, &root)) {
                printf("Part 1: %s\n", root->name);
                free_disk(root);
                ret = EXIT_FAILURE;
        }

        free_disk_list(&disks);

        return ret;
}
