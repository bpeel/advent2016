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
        int8_t x, y;
};

struct state {
        struct pos zero_pos;
        struct pos goal_pos;
};

struct history_entry {
        struct state state;
        int depth;
        bool used;
};

struct node_queue {
        struct node *first;

        int history_size;
        int history_length;
        struct history_entry *history_entries;
};

struct puzzle {
        char *board;
        int width, height;
        struct state start_state;
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
node_queue_allocate_history(struct node_queue *queue)
{
        int i;

        queue->history_entries = malloc(sizeof (struct history_entry) *
                                        queue->history_size);

        for (i = 0; i < queue->history_size; i++)
                queue->history_entries[i].used = false;
}

static void
node_queue_init(struct node_queue *queue)
{
        queue->first = NULL;

        queue->history_length = 0;
        queue->history_size = 7;
        node_queue_allocate_history(queue);
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

static uint64_t
hash_history_entry(const struct history_entry *entry)
{
        return (entry->state.zero_pos.x |
                (entry->state.zero_pos.y << 8) |
                (entry->state.goal_pos.x << 16) |
                (entry->state.goal_pos.y << 24));
}

static int
pos_equal(const struct pos *a,
          const struct pos *b)
{
        return a->x == b->x && a->y == b->y;
}

static bool
history_state_equal(const struct history_entry *a,
                    const struct history_entry *b)
{
        return (pos_equal(&a->state.zero_pos, &b->state.zero_pos) &&
                pos_equal(&a->state.goal_pos, &b->state.goal_pos));
}

static bool
node_queue_add_history_state(struct node_queue *queue,
                             const struct history_entry *state);

static void
node_queue_rehash_history(struct node_queue *queue)
{
        struct history_entry *old_entries = queue->history_entries;
        int old_size = queue->history_size;
        int i;

        queue->history_size *= 2;
        queue->history_length = 0;
        node_queue_allocate_history(queue);

        for (i = 0; i < old_size; i++) {
                if (!old_entries[i].used)
                        continue;

                node_queue_add_history_state(queue, old_entries + i);
        }

        free(old_entries);
}

static bool
node_queue_add_history_state(struct node_queue *queue,
                             const struct history_entry *state)
{
        struct history_entry *entry;
        uint64_t hash, pos;

        hash = hash_history_entry(state);
        pos = hash % queue->history_size;

        while (queue->history_entries[pos].used) {
                entry = queue->history_entries + pos;

                if (history_state_equal(entry, state)) {
                        if (entry->depth > state->depth) {
                                entry->depth = state->depth;
                                return true;
                        } else {
                                return false;
                        }
                }

                if (++pos >= queue->history_size)
                        pos = 0;
        }

        if (queue->history_length >= queue->history_size * 3 / 4) {
                node_queue_rehash_history(queue);
                return node_queue_add_history_state(queue, state);
        }

        entry = queue->history_entries + pos;
        *entry = *state;
        entry->used = true;
        queue->history_length++;

        return true;
}

static bool
node_queue_add_history(struct node_queue *queue,
                       int depth,
                       const struct state *state)
{
        struct history_entry entry;

        entry.state = *state;
        entry.depth = depth;

        return node_queue_add_history_state(queue, &entry);
}

static void
apply_move(int direction,
           struct state *state)
{
        struct pos old_pos = state->zero_pos;
        int diff = (direction & 1) ? 1 : -1;

        if ((direction & 2))
                state->zero_pos.x += diff;
        else
                state->zero_pos.y += diff;

        if (state->zero_pos.x == state->goal_pos.x &&
            state->zero_pos.y == state->goal_pos.y)
                state->goal_pos = old_pos;
}

static bool
is_valid_position(const struct puzzle *puzzle,
                  const struct pos *pos)
{
        if (pos->x < 0 || pos->x >= puzzle->width ||
            pos->y < 0 || pos->y >= puzzle->height)
                return false;

        return puzzle->board[pos->x + pos->y * puzzle->width] != '#';
}

static void
expand_position(struct node_queue *queue,
                struct node *parent,
                const struct puzzle *puzzle,
                const struct state *start_state)
{
        struct node *node;
        struct state move_state;
        int direction, depth;

        for (direction = 0; direction < N_DIRECTIONS; direction++) {
                move_state = *start_state;

                apply_move(direction, &move_state);

                if (!is_valid_position(puzzle, &move_state.zero_pos))
                        continue;

                depth = parent ? parent->depth + 1 : 1;

                if (!node_queue_add_history(queue, depth, &move_state))
                        continue;

                node = node_allocate(direction, parent);
                node_queue_add(queue, node);
                node_unref(node);
        }
}

static void
get_node_state(const struct puzzle *puzzle,
               const struct node *node,
               struct state *state)
{
        int n_moves = node->depth, i;
        int moves[n_moves];

        while (node) {
                moves[node->depth - 1] = node->direction;
                node = node->parent;
        }

        *state = puzzle->start_state;

        for (i = 0; i < n_moves; i++)
                apply_move(moves[i], state);
}

static void
expand_initial_nodes(struct node_queue *queue,
                     const struct puzzle *puzzle)
{
        expand_position(queue, NULL /* parent */, puzzle, &puzzle->start_state);
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
        struct state state = puzzle->start_state;

        while (node) {
                moves[node->depth - 1] = node->direction;
                node = node->parent;
        }

        print_position(&state.zero_pos);

        for (i = 0; i < n_moves; i++) {
                apply_move(moves[i], &state);
                printf(" %c", get_direction_name(moves[i]));
                print_position(&state.zero_pos);
        }

        printf(" (%i)\n", n_moves);

        board = malloc(puzzle->width * puzzle->height);
        memcpy(board, puzzle->board, puzzle->width * puzzle->height);

        state = puzzle->start_state;
        board[state.zero_pos.x + state.zero_pos.y * puzzle->width] = 'O';

        for (i = 0; i < n_moves; i++) {
                apply_move(moves[i], &state);
                board[state.zero_pos.x + state.zero_pos.y *
                      puzzle->width] = 'O';
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
        struct state state;
        int best_score = INT_MAX;

        node_queue_init(&queue);

        expand_initial_nodes(&queue, puzzle);

        while (queue.first) {
                node = node_queue_pop(&queue);

                if (node->depth > best_score) {
                        node_unref(node);
                        continue;
                }

                get_node_state(puzzle, node, &state);

                if (state.goal_pos.x == 0 &&
                    state.goal_pos.y == 0) {
                        best_score = node->depth;
                        print_solution(puzzle, node);
                } else {
                        expand_position(&queue, node, puzzle, &state);
                }

                node_unref(node);
        }

        node_queue_destroy(&queue);

        return best_score;
}

static void
read_board(struct puzzle *puzzle)
{
        char line[512];
        struct device {
                int x, y;
                int size;
                int used;
                struct device *next;
        };
        struct device device, *d, *devices = NULL, *next;
        char *p;
        int min_size = INT_MAX;
        int ch;

        puzzle->width = 0;
        puzzle->height = 0;

        while (fgets(line, sizeof line, stdin)) {
                if (sscanf(line,
                           "/dev/grid/node-x%i-y%i",
                           &device.x, &device.y) != 2)
                        continue;

                p = strchr(line, ' ');
                if (p == NULL)
                        continue;

                while (*p == ' ')
                        p++;

                device.size = strtoul(p, &p, 10);

                if (*p == 'T')
                        p++;

                while (*p == ' ')
                        p++;

                device.used = strtoul(p, &p, 10);

                if (device.x > puzzle->width)
                        puzzle->width = device.x;
                if (device.y > puzzle->height)
                        puzzle->height = device.y;
                if (device.size < min_size)
                        min_size = device.size;

                d = malloc(sizeof device);
                *d = device;
                d->next = devices;
                devices = d;
        }

        puzzle->width++;
        puzzle->height++;
        puzzle->board = malloc(puzzle->width * puzzle->height);

        for (d = devices; d; d = next) {
                next = d->next;

                if (d->used <= min_size)
                        ch = ' ';
                else
                        ch = '#';

                puzzle->board[d->x + d->y * puzzle->width] = ch;

                if (d->used == 0) {
                        puzzle->start_state.zero_pos.x = d->x;
                        puzzle->start_state.zero_pos.y = d->y;
                }

                free(d);
        }

        puzzle->start_state.goal_pos.x = puzzle->width - 1;
        puzzle->start_state.goal_pos.y = 0;
}

int
main(int argc, char **argv)
{
        struct puzzle puzzle;

        read_board(&puzzle);

        printf("Part 2: %i\n", solve(&puzzle));

        return 0;
}
