#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>
#include <strings.h>
#include <string.h>

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

static void
build(const struct instructions *instructions,
      char *order)
{
        int order_length = 0;
        uint32_t next_steps = instructions->root_steps;
        uint8_t completed_count[MAX_STEPS];

        memset(completed_count, 0, sizeof completed_count);

        while (next_steps) {
                int next_step = ffs(next_steps) - 1;

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
                }
        }

        order[order_length++] = '\0';
}

int
main(int argc, char **argv)
{
        struct instructions instructions;

        read_instructions(&instructions, stdin);

        char order[MAX_STEPS + 1];

        build(&instructions, order);

        printf("Part 1: %s\n", order);
}
