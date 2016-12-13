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
        int score;
        struct node *parent;
};

struct pos {
        int x, y;
};

struct history_entry {
        int depth;
        struct pos pos;
};

struct node_queue {
        int heap_size;
        int heap_length;
        struct node **heap;

        int history_size;
        int history_length;
        struct history_entry *history_entries;
};

struct puzzle {
        int favorite_num;
        int target_x;
        int target_y;
};

static const struct pos
start_pos = { 1, 1 };

static struct node *
node_allocate(int direction,
              int score,
              struct node *parent)
{
        struct node *node = malloc(sizeof (struct node));

        node->refcount = 1;
        node->direction = direction;
        node->score = score;
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
        queue->heap_length = 0;
        queue->heap_size = 8;
        queue->heap = malloc(sizeof (struct node *) * queue->heap_size);

        queue->history_length = 0;
        queue->history_size = 8;
        queue->history_entries = malloc(sizeof (struct history_entry) *
                                        queue->history_size);
}

static void
node_queue_destroy(struct node_queue *queue)
{
        int i;

        for (i = 0; i < queue->heap_length; i++)
                node_unref(queue->heap[i]);

        free(queue->heap);

        free(queue->history_entries);
}

static bool
node_less(const struct node *a,
          const struct node *b)
{
#define comp(name)             \
        if (a->name < b->name) \
                return true;   \
                               \
        if (a->name > b->name) \
                return false;

        comp(score);
        comp(depth);
        comp(direction);
#undef comp

        return false;
}

static void
node_queue_add(struct node_queue *queue,
               struct node *node)
{
        struct node *tmp;
        int pos, parent;

        if (queue->heap_size <= queue->heap_length) {
                queue->heap_size *= 2;
                queue->heap = realloc(queue->heap,
                                      sizeof (struct node *) *
                                      queue->heap_size);
        }

        pos = queue->heap_length++;
        queue->heap[pos] = node;

        while (pos > 0) {
                parent = (pos - 1) / 2;

                if (!node_less(queue->heap[pos], queue->heap[parent]))
                        break;

                tmp = queue->heap[pos];
                queue->heap[pos] = queue->heap[parent];
                queue->heap[parent] = tmp;

                pos = parent;
        }

        node->refcount++;
}

static struct node *
node_queue_pop(struct node_queue *queue)
{
        struct node *ret, *tmp;
        int pos, child, smallest;

        ret = queue->heap[0];
        queue->heap_length--;

        if (queue->heap_length <= 0)
                return ret;

        queue->heap[0] = queue->heap[queue->heap_length];

        pos = 0;

        while (true) {
                child = 2 * pos + 1;

                /* If there are no children then stop */
                if (child >= queue->heap_length)
                        break;

                /* Find the smallest of the three nodes */
                smallest = pos;

                if (node_less(queue->heap[child], queue->heap[smallest]))
                        smallest = child;

                if (child + 1 < queue->heap_length &&
                    node_less(queue->heap[child + 1], queue->heap[smallest]))
                        smallest = child + 1;

                /* If the root is already the smallest then they are
                 * in the correct order already */
                if (smallest == pos)
                        break;

                tmp = queue->heap[pos];
                queue->heap[pos] = queue->heap[smallest];
                queue->heap[smallest] = tmp;

                pos = smallest;
        }

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

static int
score_pos(const struct puzzle *puzzle,
          const struct pos *pos)
{
        return abs(pos->x - puzzle->target_x) + abs(pos->y - puzzle->target_y);
}

static void
expand_position(struct node_queue *queue,
                struct node *parent,
                const struct puzzle *puzzle,
                const struct pos *start_pos)
{
        struct node *node;
        struct pos move_pos;
        int direction, score, depth;

        for (direction = 0; direction < N_DIRECTIONS; direction++) {
                move_pos = *start_pos;

                apply_move(direction, &move_pos);

                if (!is_valid_position(puzzle, &move_pos))
                        continue;

                depth = parent ? parent->depth + 1 : 1;

                if (!node_queue_add_history(queue, depth, &move_pos))
                        continue;

                score = score_pos(puzzle, &move_pos);

                node = node_allocate(direction, score, parent);
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
                apply_move(moves[i], &pos);
                printf(" %c", get_direction_name(moves[i]));
                print_position(&pos);
        }

        printf(" (%i)\n", n_moves);
}

static void
solve(const struct puzzle *puzzle)
{
        struct node_queue queue;
        struct node *node;
        struct pos pos;
        int best_score = INT_MAX;

        node_queue_init(&queue);

        expand_initial_nodes(&queue, puzzle);

        while (queue.heap_length > 0) {
                node = node_queue_pop(&queue);

                if (node->depth > best_score) {
                        node_unref(node);
                        continue;
                }

                get_node_position(node, &pos);

                if (pos.x == puzzle->target_x && pos.y == puzzle->target_y) {
                        best_score = node->depth;
                        print_solution(node);
                } else {
                        expand_position(&queue, node, puzzle, &pos);
                }

                node_unref(node);
        }

        node_queue_destroy(&queue);
}

int
main(int argc, char **argv)
{
        struct puzzle puzzle = {
                .favorite_num = 10,
                .target_x = 7,
                .target_y = 4
        };

        if (argc >= 2)
                puzzle.favorite_num = strtol(argv[1], NULL, 10);
        if (argc >= 3)
                puzzle.target_x = strtol(argv[2], NULL, 10);
        if (argc >= 4)
                puzzle.target_y = strtol(argv[3], NULL, 10);

        solve(&puzzle);

        return 0;
}
