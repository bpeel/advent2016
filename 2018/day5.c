#include <stdint.h>
#include <stdlib.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <limits.h>

struct list_node {
        char value;
        uint16_t next;
        uint16_t prev;
};

static struct list_node *
listify(size_t polymer_length,
        const char *polymer)
{
        struct list_node *head = malloc(sizeof *head * (polymer_length + 1));

        for (size_t i = 0; i < polymer_length; i++) {
                head[i + 1].value = polymer[i];
                head[i + 1].prev = i;
                head[i + 1].next = i + 2;
        }

        head[polymer_length].next = 0;
        head->value = '*';
        head->next = 1;
        head->prev = polymer_length;

        return head;
}

static void
remove_node(struct list_node *head,
            struct list_node *node)
{
        head[node->prev].next = node->next;
        head[node->next].prev = node->prev;
        node->next = UINT16_MAX;
        node->prev = UINT16_MAX;
}

static size_t
reduce_polymer(size_t polymer_length,
               const char *polymer)
{
        struct list_node *head = listify(polymer_length, polymer);
        uint16_t n = head->next;

        /* While n is before the last element */
        while (n != 0 && head[n].next != 0) {
                /* Get the next two elements */
                char a = head[n].value;
                char b = head[head[n].next].value;

                if (!!isupper(a) != !!isupper(b) && tolower(a) == tolower(b)) {
                        /* Skip ahead 2 */
                        n = head[head[n].next].next;
                        /* Remove the previous 2 */
                        remove_node(head, head + head[n].prev);
                        remove_node(head, head + head[n].prev);
                        /* If we’re not at the beginning of the list
                         * then go back a step in case that now
                         * matches its new next node
                         */
                        if (head[n].prev != 0)
                                n = head[n].prev;
                        polymer_length -= 2;
                } else {
                        n = head[n].next;
                }
        }

        free(head);

        return polymer_length;
}

static void
strip_polymer(size_t polymer_length,
              const char *polymer_in,
              char to_strip,
              size_t *polymer_length_out,
              char **polymer_out)
{
        char *out = malloc(polymer_length);
        size_t outpos = 0;

        to_strip = tolower(to_strip);

        for (size_t i = 0; i < polymer_length; i++) {
                if (tolower(polymer_in[i]) != to_strip)
                        out[outpos++] = polymer_in[i];
        }

        *polymer_out = out;
        *polymer_length_out = outpos;
}

static void
read_polymer(FILE *in,
             char **polymer_out,
             size_t *polymer_length)
{
        size_t buf_size = 16;
        size_t buf_length = 0;
        char *buf = malloc(buf_size);

        while (true) {
                buf_length += fread(buf + buf_length,
                                    1,
                                    buf_size - buf_length,
                                    in);

                if (buf_length < buf_size)
                        break;

                buf_size *= 2;
                buf = realloc(buf, buf_size);
        }

        while (buf_length > 0 && isspace(buf[buf_length - 1]))
                buf_length--;

        *polymer_out = buf;
        *polymer_length = buf_length;
}

int
main(int argc, char **argv)
{
        char *polymer;
        size_t polymer_length;

        read_polymer(stdin, &polymer, &polymer_length);

        printf("Part 1: %i\n", (int) reduce_polymer(polymer_length, polymer));

        size_t best_length = SIZE_MAX;

        for (char ch = 'a'; ch <= 'z'; ch++) {
                char *stripped_polymer;
                size_t stripped_polymer_length;

                strip_polymer(polymer_length, polymer,
                              ch,
                              &stripped_polymer_length,
                              &stripped_polymer);

                size_t this_length = reduce_polymer(stripped_polymer_length,
                                                    stripped_polymer);

                free(stripped_polymer);

                if (this_length < best_length)
                        best_length = this_length;
        }

        printf("Part 2: %i\n", (int) best_length);

        free(polymer);

        return EXIT_SUCCESS;
}
