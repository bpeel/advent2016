#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <limits.h>
#include <stdbool.h>
#include <openssl/md5.h>

#define N_DIRECTIONS 4
#define WIDTH 4
#define HEIGHT 4

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
        struct node *first;
};

struct puzzle {
        MD5_CTX md5_base;
};

static const struct pos
start_pos = { 0, 0 };

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
is_valid_move(const struct puzzle *puzzle,
              const uint8_t *digest,
              const struct pos *end_pos,
              int direction)
{
        int digit;

        if (end_pos->x < 0 || end_pos->y < 0 ||
            end_pos->x >= WIDTH || end_pos->y >= HEIGHT)
                return false;

        digit = (digest[direction >> 1] >> ((~direction & 1) << 2)) & 0x0f;
        if (digit <= 0xa)
                return false;

        return true;
}

static int
get_direction_name(int direction)
{
        if (direction < 0 || direction >= N_DIRECTIONS)
                return '?';

        return "UDLR"[direction];
}

static void
get_digest_for_route(const struct puzzle *puzzle,
                     const struct node *end_node,
                     uint8_t *digest)
{
        const struct node *node;
        int n_moves = end_node ? end_node->depth : 0;
        char route[n_moves];
        MD5_CTX md5_ctx;

        for (node = end_node; node; node = node->parent)
                route[node->depth - 1] = get_direction_name(node->direction);

        md5_ctx = puzzle->md5_base;
        MD5_Update(&md5_ctx, route, n_moves);
        MD5_Final(digest, &md5_ctx);
}

static void
expand_position(struct node_queue *queue,
                struct node *parent,
                const struct puzzle *puzzle,
                const struct pos *start_pos)
{
        struct node *node;
        struct pos move_pos;
        int direction;
        uint8_t digest[MD5_DIGEST_LENGTH];

        get_digest_for_route(puzzle, parent, digest);

        for (direction = 0; direction < N_DIRECTIONS; direction++) {
                move_pos = *start_pos;

                apply_move(direction, &move_pos);

                if (!is_valid_move(puzzle, digest, &move_pos, direction))
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
print_solution(const struct puzzle *puzzle,
               const struct node *node)
{
        int n_moves = node->depth;
        int moves[n_moves];
        struct pos pos = start_pos;
        int i;

        while (node) {
                moves[node->depth - 1] = node->direction;
                node = node->parent;
        }

        for (i = 0; i < n_moves; i++) {
                fputc(get_direction_name(moves[i]), stdout);
                apply_move(moves[i], &pos);
        }

        printf(" %i\n", n_moves);
}

static int
solve(const struct puzzle *puzzle)
{
        struct node_queue queue;
        struct node *node;
        struct pos pos;
        int best_score = INT_MAX;

        node_queue_init(&queue);

        expand_initial_nodes(&queue, puzzle);

        while (queue.first) {
                node = node_queue_pop(&queue);

                if (node->depth > best_score) {
                        node_unref(node);
                        continue;
                }

                get_node_position(node, &pos);

                if (pos.x == WIDTH - 1 &&
                    pos.y == HEIGHT - 1) {
                        best_score = node->depth;
                        print_solution(puzzle, node);
                } else {
                        expand_position(&queue, node, puzzle, &pos);
                }

                node_unref(node);
        }

        node_queue_destroy(&queue);

        return best_score;
}

int
main(int argc, char **argv)
{
        struct puzzle puzzle;
        const char *puzzle_input = "ioramepc";

        if (argc >= 2)
                puzzle_input = argv[1];

        MD5_Init(&puzzle.md5_base);
        MD5_Update(&puzzle.md5_base, puzzle_input, strlen(puzzle_input));

        solve(&puzzle);

        return 0;
}
