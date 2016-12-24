#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <limits.h>
#include <stdbool.h>
#include <ctype.h>

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
        struct pos start;
        struct pos target;
        int width, height;
        char *board;
        int n_points;
        struct pos *points;
        int *point_distances;
};

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
        if (pos->x < 0 || pos->y < 0 ||
            pos->x >= puzzle->width ||
            pos->y >= puzzle->height)
                return false;

        return puzzle->board[pos->x + pos->y * puzzle->width] != '#';
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
get_node_position(const struct puzzle *puzzle,
                  const struct node *node,
                  struct pos *pos)
{
        int n_moves = node->depth, i;
        int moves[n_moves];

        while (node) {
                moves[node->depth - 1] = node->direction;
                node = node->parent;
        }

        *pos = puzzle->start;

        for (i = 0; i < n_moves; i++)
                apply_move(moves[i], pos);
}

static void
expand_initial_nodes(struct node_queue *queue,
                     const struct puzzle *puzzle)
{
        expand_position(queue, NULL /* parent */, puzzle, &puzzle->start);
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
print_solution(const struct puzzle *puzzle,
               const struct node *node)
{
        char *board;
        int n_moves = node->depth, i;
        int moves[n_moves];
        struct pos pos = puzzle->start;

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

        board = malloc(puzzle->width * puzzle->height);
        memcpy(board, puzzle->board, puzzle->width * puzzle->height);

        pos = puzzle->start;
        board[pos.x + pos.y * puzzle->width] = 'O';

        for (i = 0; i < n_moves; i++) {
                apply_move(moves[i], &pos);
                board[pos.x + pos.y * puzzle->width] = 'O';
        }

        for (i = 0; i < puzzle->height; i++)
                printf("%.*s\n", puzzle->width, board + i * puzzle->width);

        free(board);
}

static int
solve(const struct puzzle *puzzle)
{
        struct node_queue queue;
        struct node *node;
        struct pos pos;
        int best_score = INT_MAX;
        int ret;

        node_queue_init(&queue);

        expand_initial_nodes(&queue, puzzle);

        while (queue.first) {
                node = node_queue_pop(&queue);

                if (node->depth > best_score) {
                        node_unref(node);
                        continue;
                }

                get_node_position(puzzle, node, &pos);

                if (pos.x == puzzle->target.x &&
                    pos.y == puzzle->target.y) {
                        best_score = node->depth;
                        print_solution(puzzle, node);
                } else {
                        expand_position(&queue, node, puzzle, &pos);
                }

                node_unref(node);
        }

        ret = best_score;

        node_queue_destroy(&queue);

        return ret;
}

static void
read_board(struct puzzle *puzzle)
{
        size_t buffer_size = 16, read_offset;
        char *line;

        puzzle->board = malloc(buffer_size);
        puzzle->width = 128;
        puzzle->height = 0;

        while (true) {
                read_offset = puzzle->width * puzzle->height;
                if (read_offset + puzzle->width + 2 > buffer_size) {
                        do
                                buffer_size *= 2;
                        while (read_offset + puzzle->width + 2 > buffer_size);

                        puzzle->board = realloc(puzzle->board, buffer_size);
                }

                line = fgets(puzzle->board + read_offset,
                             buffer_size - read_offset,
                             stdin);
                if (line == NULL)
                        break;
                puzzle->width = strlen(line);
                while (puzzle->width > 0 &&
                       isspace(puzzle->board[read_offset + puzzle->width - 1]))
                        puzzle->width--;

                puzzle->height++;
        }
}

static void
read_puzzle(struct puzzle *puzzle)
{
        int n_points = 0, x, y;
        char *p;

        read_board(puzzle);

        p = puzzle->board;

        for (y = 0; y < puzzle->height; y++) {
                for (x = 0; x < puzzle->width; x++) {
                        if (*p >= '0' && *p <= '9' && *p - '0' + 1 > n_points)
                                n_points = *p - '0' + 1;
                        p++;
                }
        }

        puzzle->n_points = n_points;
        puzzle->points = malloc(sizeof (struct pos) * n_points);

        p = puzzle->board;

        for (y = 0; y < puzzle->height; y++) {
                for (x = 0; x < puzzle->width; x++) {
                        if (*p >= '0' && *p <= '9') {
                                puzzle->points[*p - '0'].x = x;
                                puzzle->points[*p - '0'].y = y;
                        }
                        p++;
                }
        }
}

static void
get_point_distances(struct puzzle *puzzle)
{
        int i, j, distance;

        puzzle->point_distances =
                malloc(sizeof (int) * puzzle->n_points * puzzle->n_points);

        for (i = 0; i < puzzle->n_points; i++) {
                for (j = 0; j < puzzle->n_points; j++) {
                        puzzle->start = puzzle->points[i];
                        puzzle->target = puzzle->points[j];
                        distance = solve(puzzle);
                        puzzle->point_distances[i * puzzle->n_points + j] =
                                distance;
                        puzzle->point_distances[j * puzzle->n_points + i] =
                                distance;
                }
        }
}

static int
score_route(const struct puzzle *puzzle,
            const int *route,
            int part)
{
        int score = 0;
        int a, b;
        int max = puzzle->n_points - 1;
        int i;

        if (part == 1)
                max++;

        for (i = 0; i < max; i++) {
                a = route[i];
                b = route[(i + 1) % puzzle->n_points];
                score += puzzle->point_distances[a * puzzle->n_points + b];
        }

        return score;
}

static void
swap(int *a, int *b)
{
        int tmp = *a;
        *a = *b;
        *b = tmp;
}

static int
find_shortest_route(const struct puzzle *puzzle,
                    int part)
{
        int *route = malloc(sizeof (int) * puzzle->n_points);
        int *permutation = route + 1;
        int permutation_length = puzzle->n_points - 1;
        int *stack = malloc(sizeof (int) * (puzzle->n_points + 1));
        int depth = 0;
        int score, best_score = INT_MAX;
        int i;

        for (i = 0; i < puzzle->n_points; i++)
                route[i] = i;
        stack[0] = -1;

        while (true) {
                if (depth >= permutation_length) {
                        score = score_route(puzzle, route, part);
                        if (score < best_score) {
                                best_score = score;
                        }
                }

                if (++stack[depth] >= permutation_length) {
                        depth--;
                        if (depth < 0)
                                break;
                        swap(permutation + stack[depth], permutation + depth);
                } else {
                        swap(permutation + stack[depth], permutation + depth);
                        depth++;
                        stack[depth] = depth - 1;
                }
        }

        free(route);
        free(stack);

        return best_score;
}

int
main(int argc, char **argv)
{
        struct puzzle puzzle;
        int part;

        read_puzzle(&puzzle);
        get_point_distances(&puzzle);

        for (part = 0; part < 2; part++) {
                printf("Part %i: %i\n",
                       part + 1,
                       find_shortest_route(&puzzle, part));
        }

        free(puzzle.board);
        free(puzzle.points);
        free(puzzle.point_distances);

        return 0;
}
