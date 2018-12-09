#include <stdint.h>
#include <stdlib.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <limits.h>
#include <string.h>
#include <inttypes.h>

struct marble {
        unsigned ccw;
        unsigned cw;
};

static void
insert_cw_after(struct marble *marbles,
                unsigned pos,
                unsigned marble)
{
        marbles[marble].ccw = pos;
        marbles[marble].cw = marbles[pos].cw;
        marbles[pos].cw = marble;
        marbles[marbles[marble].cw].ccw = marble;
}

static void
remove_marble(struct marble *marbles,
              unsigned pos)
{
        marbles[marbles[pos].ccw].cw = marbles[pos].cw;
        marbles[marbles[pos].cw].ccw = marbles[pos].ccw;
}

static struct marble *
allocate_marbles(unsigned last_marble)
{
        struct marble *marbles = malloc(sizeof *marbles * (last_marble + 1));

        marbles[0].cw = 0;
        marbles[0].ccw = 0;

        return marbles;
}

static int64_t
run(unsigned n_players,
    unsigned last_marble)
{
        struct marble *marbles = allocate_marbles(last_marble);
        unsigned current_marble = 0;
        int64_t scores[n_players];

        memset(scores, 0, sizeof scores);

        for (unsigned marble_num = 1; marble_num <= last_marble; marble_num++) {
                if (marble_num % 23 == 0) {
                        int player_num = (marble_num - 1) % n_players;

                        scores[player_num] += marble_num;

                        for (int i = 0; i < 7; i++)
                                current_marble = marbles[current_marble].ccw;

                        scores[player_num] += current_marble;

                        current_marble = marbles[current_marble].cw;

                        remove_marble(marbles, marbles[current_marble].ccw);
                } else {
                        insert_cw_after(marbles,
                                        marbles[current_marble].cw,
                                        marble_num);

                        current_marble = marble_num;
                }
        }

        free(marbles);

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
