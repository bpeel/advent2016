#include <stdint.h>
#include <stdlib.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <limits.h>
#include <string.h>
#include <inttypes.h>

struct marble {
        int value;
        struct marble *ccw;
        struct marble *cw;
};

static void
insert_cw_after(struct marble *pos,
                struct marble *marble)
{
        marble->ccw = pos;
        marble->cw = pos->cw;
        pos->cw = marble;
        marble->cw->ccw = marble;
}

static void
remove_marble(struct marble *pos)
{
        pos->ccw->cw = pos->cw;
        pos->cw->ccw = pos->ccw;
        free(pos);
}

static struct marble *
new_marble(int value)
{
        struct marble *marble = malloc(sizeof *marble);

        marble->value = value;

        return marble;
}

static void
free_marbles(struct marble *start_marble)
{
        struct marble *m = start_marble, *next;

        do {
                next = m->cw;
                free(m);
                m = next;
        } while (m != start_marble);
}

static void
print_marbles(const struct marble *start_marble)
{
#if 0
        const struct marble *lowest_marble = start_marble;

        for (const struct marble *m = start_marble->cw;
             m != start_marble;
             m = m->cw) {
                if (m->value < lowest_marble->value)
                        lowest_marble = m;
        }

        const struct marble *m = lowest_marble;

        do {
                printf("%i ", m->value);
                m = m->cw;
        } while (m != lowest_marble);

        fputc('\n', stdout);
#endif
}

static int64_t
run(int n_players,
    int n_marbles)
{
        struct marble *current_marble = new_marble(0);
        int64_t scores[n_players];

        memset(scores, 0, sizeof scores);

        current_marble->cw = current_marble;
        current_marble->ccw = current_marble;

        print_marbles(current_marble);

        for (int marble_num = 1; marble_num <= n_marbles; marble_num++) {
                if (marble_num % 23 == 0) {
                        int player_num = (marble_num - 1) % n_players;

                        scores[player_num] += marble_num;

                        for (int i = 0; i < 7; i++)
                                current_marble = current_marble->ccw;

                        scores[player_num] += current_marble->value;

                        current_marble = current_marble->cw;

                        remove_marble(current_marble->ccw);
                } else {
                        struct marble *marble = new_marble(marble_num);

                        insert_cw_after(current_marble->cw, marble);

                        current_marble = marble;
                }

                print_marbles(current_marble);
        }

        free_marbles(current_marble);

        int64_t max_score = INT64_MIN;

        for (int i = 0; i < n_players; i++) {
                if (scores[i] > max_score)
                        max_score = scores[i];
        }

        return max_score;
}

int
main(int argc, char **argv)
{
        if (argc != 3) {
                fprintf(stderr,
                        "usage: %s <n_players> <last_marble>\n",
                        argv[0]);
                return EXIT_FAILURE;
        }

        int n_players = strtol(argv[1], NULL, 10);
        int last_marble = strtol(argv[2], NULL, 10);

        int64_t score = run(n_players, last_marble);

        printf("High score: %" PRId64 "\n", score);

        return EXIT_SUCCESS;
}
