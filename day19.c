#include <stdio.h>
#include <stdlib.h>

struct elf {
        /* Index of neighouring elves in the circle */
        unsigned int next, prev;
};

static struct elf *
create_elves(int num_elves)
{
        struct elf *elves;
        int i;

        elves = malloc(num_elves * sizeof(struct elf));

        for (i = 0; i < num_elves; i++) {
                elves[i].next = i + 1;
                elves[i].prev = i - 1;
        }

        elves[0].prev = num_elves - 1;
        elves[num_elves - 1].next = 0;

        return elves;
}

static int
solve(int num_elves, int part)
{
        struct elf *elves, *thief, *victim, *neighbour;
        int victim_pos, i;
        int neighbour_pos, pos_diff;

        elves = create_elves(num_elves);
        neighbour = thief = elves;
        /* neighbor_pos is the offset from the thief to the neighbour,
         * going along next pointers */
        neighbour_pos = 0;

        while (num_elves > 1) {
                /* Victim pos relative to the thief */
                if (part == 0)
                        victim_pos = 1;
                else
                        victim_pos = num_elves / 2;

                /* Start searching from a neighbour of the previous victim */
                victim = neighbour;
                pos_diff = abs(victim_pos) - abs(neighbour_pos);

                if (abs(victim_pos) - abs(neighbour_pos) > num_elves / 2) {
                        pos_diff = num_elves - pos_diff;
                        /* Search backwards */
                        for (i = 0; i < pos_diff; i++)
                                victim = elves + victim->prev;
                } else {
                        /* Search forwards */
                        for (i = 0; i < pos_diff; i++)
                                victim = elves + victim->next;
                }

                num_elves--;

                neighbour = elves + victim->next;
                /* The neighbour pos decreases because the thief is
                 * about to move forward one */
                neighbour_pos = victim_pos - 1;
                if (neighbour_pos < 0)
                        neighbour_pos = num_elves - 1;

                elves[victim->next].prev = victim->prev;
                elves[victim->prev].next = victim->next;

                thief = elves + thief->next;
        }

        free(elves);

        return thief - elves + 1;
}

int
main(int argc, char **argv)
{
        int n_guests, i;

        if (argc != 2) {
                fprintf(stderr, "usage: day19 <n_guests>\n");
                return EXIT_FAILURE;
        }

        n_guests = strtol(argv[1], NULL, 10);

        for (i = 0; i < 2; i++)
                printf("Part %i: %i\n", i + 1, solve(n_guests, i));

        return EXIT_SUCCESS;
}
