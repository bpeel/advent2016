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

struct node_queue {
        struct node *first, *last;
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
        queue->first = queue->last = NULL;
}

static void
node_queue_destroy(struct node_queue *queue)
{
        struct node *node, *next;

        for (node = queue->first; node; node = next) {
                next = node->next;
                node_unref(node);
        }
}

static void
node_queue_add(struct node_queue *queue,
               struct node *node)
{
        node->refcount++;
        node->next = NULL;

        if (queue->first == NULL) {
                queue->first = node;
                queue->last = node;
                return;
        }

        queue->last->next = node;
        queue->last = node;
}

static struct node *
node_queue_pop(struct node_queue *queue)
{
        struct node *ret;

        ret = queue->first;

        queue->first = ret->next;
        if (queue->first == NULL)
                queue->last = NULL;

        return ret;
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
is_valid_position(int favorite_num,
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
                   favorite_num);

        bits = __builtin_popcount(pos_num);

        return (bits & 1) == 0;
}

static void
expand_position(struct node_queue *queue,
                struct node *parent,
                int favorite_num,
                const struct pos *start_pos)
{
        struct node *node;
        struct pos move_pos;
        int direction;

        for (direction = 0; direction < N_DIRECTIONS; direction++) {
                move_pos = *start_pos;

                apply_move(direction, &move_pos);

                if (!is_valid_position(favorite_num, &move_pos))
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
                     int favorite_num)
{
        expand_position(queue, NULL /* parent */, favorite_num, &start_pos);
}

static int
get_direction_name(int direction)
{
        if (direction < 0 || direction >= N_DIRECTIONS)
                return '?';

        return "UDLR"[direction];
}

static void
print_position(const struct pos *pos)
{
        printf("(%i, %i)", pos->x, pos->y);
}

static void
print_solution(const struct node *node)
{
        int n_moves = node->depth, i;
        int moves[n_moves];
        struct pos pos = start_pos;

        while (node) {
                moves[node->depth - 1] = node->direction;
                node = node->parent;
        }

        print_position(&pos);

        for (i = 0; i < n_moves; i++) {
                printf(" %c", get_direction_name(moves[i]));
                print_position(&pos);
                apply_move(moves[i], &pos);
        }

        printf(" (%i)\n", n_moves);
}

static void
solve(int favorite_num,
      int target_x,
      int target_y)
{
        struct node_queue queue;
        struct node *node;
        struct pos pos;

        node_queue_init(&queue);

        expand_initial_nodes(&queue, favorite_num);

        while (queue.first) {
                node = node_queue_pop(&queue);

                get_node_position(node, &pos);

                if (pos.x == target_x && pos.y == target_y) {
                        print_solution(node);
                        node_unref(node);
                        break;
                }

                expand_position(&queue, node, favorite_num, &pos);

                node_unref(node);
        }

        node_queue_destroy(&queue);
}

int
main(int argc, char **argv)
{
        int favorite_num = 10;
        int target_x = 7;
        int target_y = 4;

        if (argc >= 2)
                favorite_num = strtol(argv[1], NULL, 10);
        if (argc >= 3)
                target_x = strtol(argv[2], NULL, 10);
        if (argc >= 4)
                target_y = strtol(argv[3], NULL, 10);

        solve(favorite_num, target_x, target_y);

        return 0;
}
