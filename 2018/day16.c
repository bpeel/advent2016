#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#define N_REGISTERS 4

enum operation {
        OPERATION_ADD,
        OPERATION_MUL,
        OPERATION_BAN,
        OPERATION_BOR,
        OPERATION_SET,
        OPERATION_GT,
        OPERATION_EQ
};

enum source {
        SOURCE_REGISTER,
        SOURCE_IMMEDIATE
};

struct opcode {
        enum operation op;
        enum source sources[2];
};

struct cpu_state {
        int reg[N_REGISTERS];
};

struct instruction {
        int opc;
        int params[3];
};

struct observation {
        struct cpu_state before;
        struct cpu_state after;
        struct instruction instruction;
};

static const struct opcode
opcodes[16] = {
#define OPC(operation, s1, s2)                          \
        { .op = OPERATION_ ## operation,                \
          .sources = { SOURCE_ ## s1, SOURCE_ ## s2 }, }
#define OPC_R_I(operation)                      \
        OPC(operation, REGISTER, REGISTER),     \
        OPC(operation, REGISTER, IMMEDIATE)
        OPC_R_I(ADD),
        OPC_R_I(MUL),
        OPC_R_I(BAN),
        OPC_R_I(BOR),
#undef OPC_R_I
        OPC(SET, REGISTER, IMMEDIATE),
        OPC(SET, IMMEDIATE, IMMEDIATE),
        OPC(GT, IMMEDIATE, REGISTER),
        OPC(GT, REGISTER, IMMEDIATE),
        OPC(GT, REGISTER, REGISTER),
        OPC(EQ, IMMEDIATE, REGISTER),
        OPC(EQ, REGISTER, IMMEDIATE),
        OPC(EQ, REGISTER, REGISTER),
#undef OPC
};

#define N_OPCODES (sizeof opcodes / sizeof opcodes[0])

static bool
get_value(const struct cpu_state *state,
          enum source source,
          int param,
          int *value)
{
        switch (source) {
        case SOURCE_REGISTER:
                if (param < 0 || param >= N_REGISTERS)
                        return false;
                *value = state->reg[param];
                return true;
        case SOURCE_IMMEDIATE:
                *value = param;
                return true;
        }

        assert(false);

        return false;
}

static bool
apply_instruction(struct cpu_state *state,
                  int opc,
                  const int *params)
{
        if (opc < 0 || opc >= N_OPCODES)
                return false;

        const struct opcode *opcode = opcodes + opc;
        int source1, source2;

        if (!get_value(state, opcode->sources[0], params[0], &source1) ||
            !get_value(state, opcode->sources[1], params[1], &source2))
                return false;

        if (params[2] < 0 || params[2] >= N_REGISTERS)
                return false;

        int result = 0;

        switch (opcode->op) {
        case OPERATION_ADD:
                result = source1 + source2;
                break;
        case OPERATION_MUL:
                result = source1 * source2;
                break;
        case OPERATION_BAN:
                result = source1 & source2;
                break;
        case OPERATION_BOR:
                result = source1 | source2;
                break;
        case OPERATION_SET:
                result = source1;
                break;
        case OPERATION_GT:
                result = source1 > source2;
                break;
        case OPERATION_EQ:
                result = source1 == source2;
                break;
        }

        state->reg[params[2]] = result;

        return true;
}

static bool
read_observation(FILE *fin,
                 struct observation *observation)
{
        int got = fscanf(fin,
                         "Before: [%i, %i, %i, %i]\n"
                         "%i %i %i %i\n"
                         "After:  [%i, %i, %i, %i]\n",
                         &observation->before.reg[0],
                         &observation->before.reg[1],
                         &observation->before.reg[2],
                         &observation->before.reg[3],
                         &observation->instruction.opc,
                         &observation->instruction.params[0],
                         &observation->instruction.params[1],
                         &observation->instruction.params[2],
                         &observation->after.reg[0],
                         &observation->after.reg[1],
                         &observation->after.reg[2],
                         &observation->after.reg[3]);

        return got == 12;
}

static bool
read_instruction(FILE *fin,
                 struct instruction *instruction)
{
        int got = fscanf(fin,
                         "%i %i %i %i\n",
                         &instruction->opc,
                         &instruction->params[0],
                         &instruction->params[1],
                         &instruction->params[2]);

        return got == 4;
}

static bool
cpu_state_equal(const struct cpu_state *a,
                const struct cpu_state *b)
{
        for (int i = 0; i < N_REGISTERS; i++) {
                if (a->reg[i] != b->reg[i])
                        return false;
        }

        return true;
}

static int
find_opcodes(const struct observation *observation,
             int *possible_opcodes_out)
{
        int possible_opcodes = 0;
        int count = 0;

        for (int opc = 0; opc < N_OPCODES; opc++) {
                struct cpu_state cpu = observation->before;

                if (!apply_instruction(&cpu,
                                       opc,
                                       observation->instruction.params))
                        continue;

                if (cpu_state_equal(&observation->after, &cpu)) {
                        count++;
                        possible_opcodes |= 1 << opc;
                }
        }

        *possible_opcodes_out = possible_opcodes;

        return count;
}

static bool
is_pot(int x)
{
        return x && (x & (x-1)) == 0;
}

static int
find_one_possible_opcode(const int *possible_opcodes_map)
{
        for (int i = 0; i < N_OPCODES; i++) {
                if (is_pot(possible_opcodes_map[i]))
                        return i;
        }

        return -1;
}

static bool
build_opcode_map(const int *possible_opcodes_map_in,
                 int *opcode_map)
{
        int possible_opcodes_map[N_OPCODES];

        memcpy(possible_opcodes_map,
               possible_opcodes_map_in,
               sizeof possible_opcodes_map);

        for (int count = 0; count < N_OPCODES; count++) {
                int next_one = find_one_possible_opcode(possible_opcodes_map);

                if (next_one == -1) {
                        fprintf(stderr,
                                "Run out of one opcode mappings\n");
                        return false;
                }

                int mapped_opc = ffs(possible_opcodes_map[next_one]) - 1;
                opcode_map[next_one] = mapped_opc;

                possible_opcodes_map[next_one] = 0;

                for (int opc = 0; opc < N_OPCODES; opc++)
                        possible_opcodes_map[opc] &= ~(1 << mapped_opc);
        }

        return true;
}

int
main(int argc, char **argv)
{
        struct observation observation;
        int n_samples = 0;
        int possible_opcodes_map[N_OPCODES];

        for (int i = 0; i < N_OPCODES; i++)
                possible_opcodes_map[i] = (1 << N_OPCODES) - 1;

        while (read_observation(stdin, &observation)) {
                int possible_opcodes;
                int n_opcodes = find_opcodes(&observation, &possible_opcodes);

                if (n_opcodes >= 3)
                        n_samples++;

                if (observation.instruction.opc < 0 ||
                    observation.instruction.opc >= N_OPCODES) {
                        fprintf(stderr,
                                "Encountered opcode %i\n",
                                observation.instruction.opc);
                        return EXIT_FAILURE;
                }

                possible_opcodes_map[observation.instruction.opc] &=
                        possible_opcodes;
        }

        printf("Part 1: %i\n", n_samples);

        int opcode_map[N_OPCODES];

        if (!build_opcode_map(possible_opcodes_map, opcode_map))
                return EXIT_FAILURE;

        struct instruction instruction;
        struct cpu_state cpu = { .reg = { 0 } };

        while (read_instruction(stdin, &instruction)) {
                if (!apply_instruction(&cpu,
                                       opcode_map[instruction.opc],
                                       instruction.params)) {
                        fprintf(stderr,
                                "Tried to execute invalid instruction\n");
                        return EXIT_FAILURE;
                }
        }

        printf("Part 2: %i\n", cpu.reg[0]);
}
