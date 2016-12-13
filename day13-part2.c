#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <limits.h>
#include <stdbool.h>

#define N_DIRECTIONS 4

struct node {
        int direction;
        int refcount;
        int depth;
        struct node *parent;
        struct node *next;
};

struct pos {
        int x, y;
};

struct history_entry {
        int depth;
        struct pos pos;
};

struct node_queue {
        struct node *first;

        int history_size;
        int history_length;
        struct history_entry *history_entries;
};

struct puzzle {
        int favorite_num;
        int max_moves;
};

static const struct pos
start_pos = { 1, 1 };

static struct node *
node_allocate(int direction,
              struct node *parent)
{
        struct node *node = malloc(sizeof (struct node));

        node->refcount = 1;
        node->direction = direction;
        node->parent = parent;

        if (parent) {
                parent->refcount++;
                node->depth = parent->depth + 1;
        } else {
                node->depth = 1;
        }

        return node;
}

static void
node_unref(struct node *node)
{
        struct node *parent;

        while (--node->refcount <= 0) {
                parent = node->parent;
                free(node);
                node = parent;
                if (node == NULL)
                        break;
        }
}

static void
node_queue_init(struct node_queue *queue)
{
        queue->first = NULL;

        queue->history_length = 0;
        queue->history_size = 8;
        queue->history_entries = malloc(sizeof (struct history_entry) *
                                        queue->history_size);
}

static void
node_queue_destroy(struct node_queue *queue)
{
        struct node *node, *next;

        for (node = queue->first; node; node = next) {
                next = node->next;
                node_unref(node);
        }

        free(queue->history_entries);
}

static void
node_queue_add(struct node_queue *queue,
               struct node *node)
{
        node->next = queue->first;
        queue->first = node;
        node->refcount++;
}

static struct node *
node_queue_pop(struct node_queue *queue)
{
        struct node *ret;

        ret = queue->first;
        queue->first = ret->next;

        return ret;
}

static int
compare_pos(const struct pos *a,
            const struct pos *b)
{
        if (a->x < b->x)
                return -1;

        if (a->x > b->x)
                return 1;

        if (a->y < b->y)
                return -1;

        if (a->y > b->y)
                return 1;

        return 0;
}

static bool
node_queue_add_history(struct node_queue *queue,
                       int depth,
                       const struct pos *pos)
{
        struct history_entry *entry;
        int min = 0, max = queue->history_length, mid;
        int comp;

        while (min < max) {
                mid = (min + max) / 2;
                entry = queue->history_entries + mid;

                comp = compare_pos(&entry->pos, pos);

                if (comp == 0) {
                        if (entry->depth > depth) {
                                entry->depth = depth;
                                return true;
                        } else {
                                return false;
                        }
                } else if (comp < 0) {
                        min = mid + 1;
                } else {
                        max = mid;
                }
        }

        if (queue->history_length >= queue->history_size) {
                queue->history_size *= 2;
                queue->history_entries =
                        realloc(queue->history_entries,
                                sizeof (struct history_entry) *
                                queue->history_size);
        }

        memmove(queue->history_entries + min + 1,
                queue->history_entries + min,
                (queue->history_length - min) * sizeof (struct history_entry));

        entry = queue->history_entries + min;
        entry->pos = *pos;
        entry->depth = depth;
        queue->history_length++;

        return true;
}

static void
apply_move(int direction,
           struct pos *pos)
{
        int diff = (direction & 1) ? 1 : -1;

        if ((direction & 2))
                pos->x += diff;
        else
                pos->y += diff;
}

static bool
is_valid_position(const struct puzzle *puzzle,
                  const struct pos *pos)
{
        int pos_num, bits;

        if (pos->x < 0 || pos->y < 0)
                return false;

        pos_num = (pos->x * pos->x +
                   3 * pos->x +
                   2 * pos->x * pos->y +
                   pos->y +
                   pos->y * pos->y +
                   puzzle->favorite_num);

        bits = __builtin_popcount(pos_num);

        return (bits & 1) == 0;
}

static void
expand_position(struct node_queue *queue,
                struct node *parent,
                const struct puzzle *puzzle,
                const struct pos *start_pos)
{
        struct node *node;
        struct pos move_pos;
        int direction, depth;

        for (direction = 0; direction < N_DIRECTIONS; direction++) {
                move_pos = *start_pos;

                apply_move(direction, &move_pos);

                if (!is_valid_position(puzzle, &move_pos))
                        continue;

                depth = parent ? parent->depth + 1 : 1;

                if (!node_queue_add_history(queue, depth, &move_pos))
                        continue;

                node = node_allocate(direction, parent);
                node_queue_add(queue, node);
                node_unref(node);
        }
}

static void
get_node_position(const struct node *node,
                  struct pos *pos)
{
        int n_moves = node->depth, i;
        int moves[n_moves];

        while (node) {
                moves[node->depth - 1] = node->direction;
                node = node->parent;
        }

        *pos = start_pos;

        for (i = 0; i < n_moves; i++)
                apply_move(moves[i], pos);
}

static void
expand_initial_nodes(struct node_queue *queue,
                     const struct puzzle *puzzle)
{
        expand_position(queue, NULL /* parent */, puzzle, &start_pos);
}

static void
solve(const struct puzzle *puzzle)
{
        struct node_queue queue;
        struct node *node;
        struct pos pos;

        node_queue_init(&queue);

        expand_initial_nodes(&queue, puzzle);

        while (queue.first) {
                node = node_queue_pop(&queue);

                if (node->depth < 50) {
                        get_node_position(node, &pos);
                        expand_position(&queue, node, puzzle, &pos);
                }

                node_unref(node);
        }

        printf("%i\n", queue.history_length);

        node_queue_destroy(&queue);
}

int
main(int argc, char **argv)
{
        struct puzzle puzzle = {
                .favorite_num = 10,
                .max_moves = 50
        };

        if (argc >= 2)
                puzzle.favorite_num = strtol(argv[1], NULL, 10);
        if (argc >= 3)
                puzzle.max_moves = strtol(argv[2], NULL, 10);

        solve(&puzzle);

        return 0;
}
