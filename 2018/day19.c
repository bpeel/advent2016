#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <ctype.h>

#define N_REGISTERS 6

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
        const char *name;
};

struct cpu_state {
        int reg[N_REGISTERS];
};

struct instruction {
        int opc;
        int params[3];
};

static const struct opcode
opcodes[16] = {
#define OPC(operation, s1, s2, name_str)                \
        { .op = OPERATION_ ## operation,                \
          .sources = { SOURCE_ ## s1, SOURCE_ ## s2 },  \
          .name = name_str }
#define OPC_R_I(operation, name)                        \
        OPC(operation, REGISTER, REGISTER, name "r"),   \
        OPC(operation, REGISTER, IMMEDIATE, name "i")
        OPC_R_I(ADD, "add"),
        OPC_R_I(MUL, "mul"),
        OPC_R_I(BAN, "ban"),
        OPC_R_I(BOR, "bor"),
#undef OPC_R_I
        OPC(SET, REGISTER, IMMEDIATE, "setr"),
        OPC(SET, IMMEDIATE, IMMEDIATE, "seti"),
        OPC(GT, IMMEDIATE, REGISTER, "gtir"),
        OPC(GT, REGISTER, IMMEDIATE, "gtri"),
        OPC(GT, REGISTER, REGISTER, "gtrr"),
        OPC(EQ, IMMEDIATE, REGISTER, "eqir"),
        OPC(EQ, REGISTER, IMMEDIATE, "eqri"),
        OPC(EQ, REGISTER, REGISTER, "eqrr"),
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
                  const struct instruction *instruction)
{
        if (instruction->opc < 0 || instruction->opc >= N_OPCODES)
                return false;

        const struct opcode *opcode = opcodes + instruction->opc;
        int source1, source2;

        if (!get_value(state,
                       opcode->sources[0],
                       instruction->params[0],
                       &source1) ||
            !get_value(state,
                       opcode->sources[1],
                       instruction->params[1],
                       &source2))
                return false;

        if (instruction->params[2] < 0 || instruction->params[2] >= N_REGISTERS)
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

        state->reg[instruction->params[2]] = result;

        return true;
}

static bool
read_instruction(FILE *fin,
                 struct instruction *instruction)
{
        char opname[16];
        int got = fscanf(fin,
                         "%15s %i %i %i\n",
                         opname,
                         &instruction->params[0],
                         &instruction->params[1],
                         &instruction->params[2]);

        if (got != 4)
                return false;

        for (int i = 0; i < N_OPCODES; i++) {
                if (!strcmp(opname, opcodes[i].name)) {
                        instruction->opc = i;
                        return true;
                }
        }

        return false;
}

static bool
read_instructions(FILE *fin,
                  size_t *n_instructions_out,
                  struct instruction **instructions_out)
{
        size_t buf_size = 1;
        struct instruction *instructions =
                malloc(buf_size * sizeof *instructions);
        size_t n_instructions = 0;

        while (true) {
                if (n_instructions >= buf_size) {
                        buf_size *= 2;
                        instructions = realloc(instructions,
                                               buf_size * sizeof *instructions);
                }

                if (!read_instruction(fin, instructions + n_instructions))
                        break;

                n_instructions++;
        }

        *n_instructions_out = n_instructions;
        *instructions_out = instructions;

        return true;
}

static int
read_ip_register(FILE *in)
{
        int reg;
        int got = fscanf(in, "#ip %i\n", &reg);

        if (got != 1 || reg < 0 || reg >= N_REGISTERS) {
                fprintf(stderr, "Invalid #ip line\n");
                return -1;
        }

        return reg;
}

static void
print_regs(const struct cpu_state *cpu)
{
        fputc('[', stdout);
        for (int i = 0; i < N_REGISTERS; i++) {
                if (i > 0)
                        fputs(", ", stdout);
                printf("%i", cpu->reg[i]);
        }
        fputc(']', stdout);
}

static void
print_instruction(const struct instruction *instruction)
{
        if (instruction->opc < 0 || instruction->opc > N_OPCODES)
                printf(" (%i)", instruction->opc);
        else
                printf(" %s", opcodes[instruction->opc].name);

        for (int i = 0; i < 3; i++)
                printf(" %i", instruction->params[i]);
}

static bool
run_program(bool trace,
            struct cpu_state *cpu,
            int ip_reg,
            size_t n_instructions,
            const struct instruction *instructions)
{
        while (cpu->reg[ip_reg] >= 0 && cpu->reg[ip_reg] < n_instructions) {
                if (trace) {
                        print_regs(cpu);
                        print_instruction(instructions + cpu->reg[ip_reg]);
                }

                if (!apply_instruction(cpu, instructions + cpu->reg[ip_reg])) {
                        fprintf(stderr, "Invalid instruction encountered\n");
                        return false;
                }

                if (trace) {
                        fputc(' ', stdout);
                        print_regs(cpu);
                        fputc('\n', stdout);
                }

                cpu->reg[ip_reg]++;
        }

        return true;
}

int
main(int argc, char **argv)
{
        bool trace = argc > 1;

        int ip_reg = read_ip_register(stdin);

        if (ip_reg == -1)
                return EXIT_FAILURE;

        size_t n_instructions;
        struct instruction *instructions;

        if (!read_instructions(stdin, &n_instructions, &instructions))
                return EXIT_FAILURE;

        struct cpu_state cpu = { .reg = { 0 } };

        if (run_program(trace, &cpu, ip_reg, n_instructions, instructions)) {
                printf("Part 1: %i\n", cpu.reg[0]);

                memset(&cpu, 0, sizeof cpu);
                cpu.reg[0] = 1;

                if (run_program(trace,
                                &cpu,
                                ip_reg,
                                n_instructions,
                                instructions)) {
                        printf("Part 2: %i\n", cpu.reg[0]);
                }
        }

        free(instructions);
}
