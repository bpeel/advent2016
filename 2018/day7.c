#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>
#include <strings.h>
#include <string.h>
#include <limits.h>

#define MAX_STEPS (sizeof (uint32_t) * 8)

struct step {
        /* Bit mask of steps that have this step as a requirement */
        uint32_t children;
        uint8_t n_parents;
};

struct instructions {
        /* Bit mask of steps that have no parents */
        uint32_t root_steps;
        struct step steps[MAX_STEPS];
};

struct parse_data {
        struct instructions *instructions;
        uint32_t seen_rules;
};

struct elf {
        /* The next time point that this elf will be available */
        int next_free_time;
};

static void
see_rule(struct parse_data *data,
         int rule_num)
{
        if ((data->seen_rules & (1 << rule_num)))
                return;

        data->seen_rules |= 1 << rule_num;

        struct step *step = data->instructions->steps + rule_num;
        step->children = 0;
        step->n_parents = 0;

        data->instructions->root_steps |= (1 << rule_num);
}

static void
read_instructions(struct instructions *instructions,
                  FILE *in)
{
        struct parse_data data = {
                .instructions = instructions,
                .seen_rules = 0
        };

        instructions->root_steps = 0;

        while (true) {
                char parent_letter, child_letter;
                int ret = fscanf(in,
                                 "Step %c must be finished before "
                                 "step %c can begin.\n",
                                 &parent_letter,
                                 &child_letter);

                if (ret < 2)
                        break;

                int parent = parent_letter - 'A';
                int child = child_letter - 'A';

                see_rule(&data, parent);
                see_rule(&data, child);

                instructions->steps[parent].children |= 1 << child;
                instructions->steps[child].n_parents++;
                instructions->root_steps &= ~(1 << child);
        }
}

static int
get_next_elf(int n_elves,
             const struct elf *elves)
{
        /* Find the elf with the lowest next free time */
        int lowest_time = INT_MAX;
        int lowest_index = 0;

        for (int i = 0; i < n_elves; i++) {
                if (elves[i].next_free_time < lowest_time) {
                        lowest_time = elves[i].next_free_time;
                        lowest_index = i;
                }
        }

        return lowest_index;
}

static int
task_time(int task_num)
{
        return task_num + 61;
}

static int
find_next_step(uint32_t next_steps,
               const int *step_available_time)
{
        int min_time = INT_MAX;
        int min_step = 0;

        while (next_steps) {
                int step = ffs(next_steps) - 1;

                if (step_available_time[step] < min_time) {
                        min_time = step_available_time[step];
                        min_step = step;
                }

                next_steps &= ~(1 << step);
        }

        return min_step;
}

static void
build(const struct instructions *instructions,
      char *order,
      int n_elves,
      int *elapsed_time_out)
{
        int order_length = 0;
        uint32_t next_steps = instructions->root_steps;
        uint8_t completed_count[MAX_STEPS];
        int step_available_time[MAX_STEPS];
        struct elf elves[n_elves];
        int current_time = 0;

        memset(completed_count, 0, sizeof completed_count);
        memset(step_available_time, 0, sizeof step_available_time);
        memset(elves, 0, sizeof (struct elf) * n_elves);

        while (next_steps) {
                int next_step;

                if (n_elves == 1) {
                        next_step = ffs(next_steps) - 1;
                } else {
                        next_step = find_next_step(next_steps,
                                                   step_available_time);
                }

                struct elf *next_elf = elves + get_next_elf(n_elves, elves);

                if (next_elf->next_free_time > current_time)
                        current_time = next_elf->next_free_time;
                if (step_available_time[next_step] > current_time)
                        current_time = step_available_time[next_step];

                int completion_time = current_time + task_time(next_step);

                next_elf->next_free_time = completion_time;

                order[order_length++] = next_step + 'A';
                next_steps &= ~(1 << next_step);

                uint32_t children = instructions->steps[next_step].children;

                while (children) {
                        int child = ffs(children) - 1;
                        completed_count[child]++;
                        if (completed_count[child] >=
                            instructions->steps[child].n_parents) {
                                next_steps |= 1 << child;
                        }
                        children &= ~(1 << child);

                        if (step_available_time[child] < completion_time)
                                step_available_time[child] = completion_time;
                }
        }

        order[order_length++] = '\0';

        int total_time = 0;

        for (int i = 0; i < n_elves; i++) {
                if (elves[i].next_free_time > total_time)
                        total_time = elves[i].next_free_time;
        }

        *elapsed_time_out = total_time;
}

int
main(int argc, char **argv)
{
        struct instructions instructions;

        read_instructions(&instructions, stdin);

        char order[MAX_STEPS + 1];
        int elapsed_time;

        build(&instructions, order, 1, &elapsed_time);

        printf("Part 1: %s\n", order);

        build(&instructions, order, 5, &elapsed_time);

        printf("Part 2: %i\n", elapsed_time);
}
