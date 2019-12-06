#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>

#include "pcx-buffer.h"

struct object {
        char name[4];
        char parent_name[4];
        int first_child;
        int next_sibling;
};

struct stack_entry {
        int object_num;
        int depth;
};

static bool
parse_object(struct object *obj,
             const char *line)
{
        const char *split = strchr(line, ')');

        if (split == NULL || split == line || split - line >= sizeof obj->name)
                goto error;

        memcpy(obj->parent_name, line, split - line);
        obj->parent_name[split - line] = '\0';

        const char *name = split + 1;
        size_t name_len = strlen(name);

        while (name_len > 0 && isspace(name[name_len - 1]))
                name_len--;

        if (name_len < 1 || name_len >= sizeof obj->name)
                goto error;

        memcpy(obj->name, name, name_len);
        obj->name[name_len] = '\0';

        obj->first_child = -1;
        obj->next_sibling = -1;

        return true;

error:
        fprintf(stderr, "Invalid object: %s", line);
        return false;
}

static bool
read_objects(FILE *in,
             size_t *n_objects_out,
             struct object **objects_out)
{
        char line[512];
        struct pcx_buffer buf = PCX_BUFFER_STATIC_INIT;

        while (fgets(line, sizeof line, in)) {
                pcx_buffer_set_length(&buf,
                                      buf.length + sizeof (struct object));
                if (!parse_object((struct object *)
                                  (buf.data +
                                   buf.length -
                                   sizeof (struct object)),
                                  line))
                    goto error;
        }

        /* Add the special COM object which has no parent */
        pcx_buffer_set_length(&buf, buf.length + sizeof (struct object));
        struct object *com = (struct object *) (buf.data +
                                                buf.length -
                                                sizeof (struct object));
        strcpy(com->name, "COM");
        com->parent_name[0] = '\0';
        com->first_child = -1;
        com->next_sibling = -1;

        *n_objects_out = buf.length / sizeof (struct object);
        *objects_out = (struct object *) buf.data;

        return true;

error:
        pcx_buffer_destroy(&buf);
        return false;
}

static int
compare_object_name(const void *pa, const void *pb)
{
        const struct object *a = pa;
        const struct object *b = pb;

        return strcmp(a->name, b->name);
}

static struct object *
find_object(size_t n_objects,
            const struct object *objects,
            const char *name)
{
        _Static_assert((void *) objects->name == (void *) objects);

        return bsearch(name,
                       objects,
                       n_objects,
                       sizeof *objects,
                       compare_object_name);
}

static bool
build_tree(size_t n_objects,
           struct object *objects)
{
        for (unsigned i = 0; i < n_objects; i++) {
                struct object *obj = objects + i;

                if (i > 0 && !strcmp(obj->name, objects[i - 1].name)) {
                        fprintf(stderr,
                                "object %s appears twice\n",
                                objects[i].name);
                        return false;
                }

                /* Don’t look for a parent for the root object */
                if (obj->parent_name[0] == '\0')
                        continue;

                struct object *parent = find_object(n_objects,
                                                    objects,
                                                    obj->parent_name);

                if (parent == NULL) {
                        fprintf(stderr,
                                "missing object %s\n",
                                obj->parent_name);
                        return false;
                }

                if (parent == obj) {
                        fprintf(stderr,
                                "%s is its own parent\n",
                                obj->name);
                        return false;
                }

                obj->next_sibling = parent->first_child;
                parent->first_child = i;
        }

        return true;
}

static void
add_to_stack(struct pcx_buffer *stack,
             int object_num,
             int depth)
{
        pcx_buffer_set_length(stack,
                              stack->length + sizeof (struct stack_entry));
        struct stack_entry *entry = ((struct stack_entry *)
                                     (stack->data +
                                      stack->length -
                                      sizeof (struct stack_entry)));
        entry->object_num = object_num;
        entry->depth = depth;
}

static int
count_orbits(size_t n_objects,
             const struct object *objects)
{
        const struct object *root = find_object(n_objects, objects, "COM");

        assert(root);

        int total_orbits = 0;
        struct pcx_buffer stack = PCX_BUFFER_STATIC_INIT;

        add_to_stack(&stack, root - objects, 0);

        while (stack.length > 0) {
                struct stack_entry entry = *((struct stack_entry *)
                                             (stack.data +
                                              stack.length -
                                              sizeof entry));
                stack.length -= sizeof entry;

                total_orbits += entry.depth;

                const struct object *obj = objects + entry.object_num;

                if (obj->next_sibling >= 0)
                        add_to_stack(&stack, obj->next_sibling, entry.depth);

                if (obj->first_child >= 0)
                        add_to_stack(&stack, obj->first_child, entry.depth + 1);
        }

        pcx_buffer_destroy(&stack);

        return total_orbits;
}

int
main(int argc, char **argv)
{
        size_t n_objects;
        struct object *objects;

        if (!read_objects(stdin, &n_objects, &objects))
                return EXIT_FAILURE;

        qsort(objects, n_objects, sizeof *objects, compare_object_name);

        int ret = EXIT_SUCCESS;

        if (!build_tree(n_objects, objects)) {
                ret = EXIT_FAILURE;
                goto out;
        }

        printf("Part 1: %i\n", count_orbits(n_objects, objects));

out:
        pcx_free(objects);

        return ret;
}
