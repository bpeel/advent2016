#include <stdlib.h>
#include <stdio.h>
#include <limits.h>

struct node {
        struct node *next;
        int n_children;
        struct node **children;
        int value;
        int n_meta;
        int meta[];
};

static void
free_node(struct node *node)
{
        for (int i = 0; i < node->n_children; i++) {
                if (node->children[i])
                        free_node(node->children[i]);
        }

        free(node->children);
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
        node->value = INT_MAX;
        node->n_children = n_children;
        node->children = calloc(node->n_children, sizeof (struct node *));
        node->n_meta = n_meta;

        for (int i = 0; i < n_children; i++) {
                node->children[i] = read_node(in);

                if (node->children[i] == NULL) {
                        free_node(node);
                        return NULL;
                }
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

        for (int i = 0; i < node->n_children; i++)
                sum += sum_meta(node->children[i]);

        return sum;
}

static int
node_value(struct node *node)
{
        if (node->value != INT_MAX)
                return node->value;

        int sum = 0;

        if (node->n_children <= 0) {
                for (int i = 0; i < node->n_meta; i++)
                        sum += node->meta[i];
        } else {
                for (int i = 0; i < node->n_meta; i++) {
                        int meta = node->meta[i];

                        if (meta < 1 || meta > node->n_children)
                                continue;

                        sum += node_value(node->children[meta - 1]);
                }
        }

        node->value = sum;

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
        printf("Part 2: %i\n", node_value(node));

        free_node(node);

        return EXIT_SUCCESS;
}
