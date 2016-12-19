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
        struct elf *thief, *victim;
        int victim_pos, i;

        thief = create_elves(num_elves);

        while (num_elves > 1) {
                victim_pos = num_elves / 2;
                for (victim = thief, i = 0; i < victim_pos; i++)
                        victim = victim->next;

                thief->presents += victim->presents;

                victim->next->prev = victim->prev;
                victim->prev->next = victim->next;
                free(victim);

                num_elves--;

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
