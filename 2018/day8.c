#include <stdlib.h>
#include <stdio.h>

struct node {
        struct node *next;
        struct node *first_child;
        int n_meta;
        int meta[];
};

static void
free_node(struct node *node)
{
        struct node *child, *next;

        for (child = node->first_child; child; child = next) {
                next = child->next;
                free_node(child);
        }

        free(node);
}

static struct node *
read_node(FILE *in)
{
        int n_children, n_meta;

        if (scanf(" %i %i", &n_children, &n_meta) != 2)
                return NULL;

        struct node *node = malloc(sizeof *node +
                                   sizeof (node->meta[0]) * n_meta);

        node->next = NULL;
        node->first_child = NULL;
        node->n_meta = n_meta;

        for (int i = 0; i < n_children; i++) {
                struct node *child = read_node(in);

                if (child == NULL) {
                        free_node(node);
                        return NULL;
                }

                child->next = node->first_child;
                node->first_child = child;
        }

        for (int i = 0; i < n_meta; i++) {
                if (scanf(" %i", node->meta + i) != 1) {
                        free_node(node);
                        return NULL;
                }
        }

        return node;
}

static int
sum_meta(const struct node *node)
{
        int sum = 0;

        for (int i = 0; i < node->n_meta; i++)
                sum += node->meta[i];

        for (struct node *child = node->first_child; child; child = child->next)
                sum += sum_meta(child);

        return sum;
}

int
main(int argc, char **argv)
{
        struct node *node = read_node(stdin);

        if (node == NULL) {
                fprintf(stderr, "error reading node\n");
                return EXIT_FAILURE;
        }

        printf("Part 1: %i\n", sum_meta(node));

        free_node(node);

        return EXIT_SUCCESS;
}
