#include <stdio.h>
#include <stdlib.h>

struct elf {
        int num;
        int presents;
        struct elf *next, *prev;
};

static struct elf *
alloc_elf(int num)
{
        struct elf *elf;

        elf = malloc(sizeof (struct elf));
        elf->num = num;
        elf->presents = 1;

        return elf;
}

static struct elf *
create_elves(int num_elves)
{
        struct elf *elf, *last_elf, *first_elf;
        int i;

        first_elf = last_elf = alloc_elf(1);

        for (i = 1; i < num_elves; i++) {
                elf = alloc_elf(i + 1);
                elf->prev = last_elf;
                last_elf->next = elf;
                last_elf = elf;
        }

        last_elf->next = first_elf;
        first_elf->prev = last_elf;

        return first_elf;
}

static void
solve(int num_elves)
{
        struct elf *thief, *victim, *neighbour;
        int victim_pos, i;
        int neighbour_pos, pos_diff;

        thief = create_elves(num_elves);
        neighbour = thief;
        /* neighbor_pos is the offset from the thief to the neighbour,
         * going along next pointers */
        neighbour_pos = 0;

        while (num_elves > 1) {
                /* Victim pos relative to the thief */
                victim_pos = num_elves / 2;

                /* Start searching from a neighbour of the previous victim */
                victim = neighbour;
                pos_diff = abs(victim_pos) - abs(neighbour_pos);

                if (abs(victim_pos) - abs(neighbour_pos) > num_elves / 2) {
                        pos_diff = num_elves - pos_diff;
                        /* Search backwards */
                        for (i = 0; i < pos_diff; i++)
                                victim = victim->prev;
                } else {
                        /* Search forwards */
                        for (i = 0; i < pos_diff; i++)
                                victim = victim->next;
                }

                thief->presents += victim->presents;

                num_elves--;

                neighbour = victim->next;
                /* The neighbour pos decreases because the thief is
                 * about to move forward one */
                neighbour_pos = victim_pos - 1;
                if (neighbour_pos < 0)
                        neighbour_pos = num_elves - 1;

                victim->next->prev = victim->prev;
                victim->prev->next = victim->next;
                free(victim);

                thief = thief->next;
        }

        printf("%i wins\n", thief->num);

        free(thief);
}

int
main(int argc, char **argv)
{
        if (argc != 2) {
                fprintf(stderr, "usage: day19 <n_guests>\n");
                return EXIT_FAILURE;
        }

        solve(strtol(argv[1], NULL, 10));

        return EXIT_SUCCESS;
}
