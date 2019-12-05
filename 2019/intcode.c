#include "intcode.h"

#include <stdbool.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdio.h>

#include "pcx-util.h"

#define MAX_PARAMS 3

struct pcx_error_domain
intcode_error_domain;

struct intcode {
        size_t memory_size;
        int64_t *memory;
        int64_t pc;
        bool stopped;
};

struct opcode {
        int n_params;
        bool (* func)(struct intcode *machine,
                      const int64_t *params,
                      struct pcx_error **error);
};

static bool
store_result(struct intcode *machine,
             int64_t result,
             struct pcx_error **error)
{
        if (!intcode_write_indirect(machine,
                                    machine->pc,
                                    result,
                                    error))
                return false;

        machine->pc++;

        return true;
}

#define ALU_OP(name, code)                                      \
        static bool                                             \
        opcode_ ## name(struct intcode *machine,                \
                        const int64_t *params,                  \
                        struct pcx_error **error)               \
        {                                                       \
                int64_t a = params[0], b = params[1];           \
                                                                \
                return store_result(machine, (code), error);    \
        }

ALU_OP(add, a + b);
ALU_OP(multiply, a * b);

#undef ALU_OP

static bool
opcode_stop(struct intcode *machine,
            const int64_t *params,
            struct pcx_error **error)
{
        machine->stopped = true;

        return true;
}

static bool
opcode_input(struct intcode *machine,
             const int64_t *params,
             struct pcx_error **error)
{
        int value;

        int got = fscanf(stdin, "%d", &value);

        if (got != 1) {
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_IO,
                              "Error getting input");
                return false;
        }

        return store_result(machine, value, error);
}

static bool
opcode_output(struct intcode *machine,
              const int64_t *params,
              struct pcx_error **error)
{
        fprintf(stdout, "%" PRIi64 "\n", params[0]);
        return true;
}

static bool
opcode_jump_true(struct intcode *machine,
                 const int64_t *params,
                 struct pcx_error **error)
{
        if (params[0])
                machine->pc = params[1];

        return true;
}

static bool
opcode_jump_false(struct intcode *machine,
                  const int64_t *params,
                  struct pcx_error **error)
{
        if (!params[0])
                machine->pc = params[1];

        return true;
}

static const struct opcode
opcodes[] = {
        [1] = { 2, opcode_add },
        [2] = { 2, opcode_multiply },
        [3] = { 0, opcode_input },
        [4] = { 1, opcode_output },
        [5] = { 2, opcode_jump_true },
        [6] = { 2, opcode_jump_false },
        [99] = { 0, opcode_stop }
};

static bool
check_address(const struct intcode *machine,
              int64_t address,
              const char *action,
              struct pcx_error **error)
{
        if (address < 0 || address >= machine->memory_size) {
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_INVALID_ADDRESS,
                              "Invalid %s %" PRIi64,
                              action,
                              address);
                return false;
        }

        return true;
}

bool
intcode_read(const struct intcode *machine,
             int64_t address,
             int64_t *value,
             struct pcx_error **error)
{
        if (!check_address(machine, address, "read from", error))
                return false;

        *value = machine->memory[address];

        return true;
}

bool
intcode_write(struct intcode *machine,
              int64_t address,
              int64_t value,
              struct pcx_error **error)
{
        if (!check_address(machine, address, "write to", error))
                return false;

        machine->memory[address] = value;

        return true;
}

bool
intcode_read_indirect(const struct intcode *machine,
                      int64_t address,
                      int64_t *value,
                      struct pcx_error **error)
{
        return (intcode_read(machine,
                             address,
                             &address,
                             error) &&
                intcode_read(machine,
                             address,
                             value,
                             error));
}

bool
intcode_write_indirect(struct intcode *machine,
                       int64_t address,
                       int64_t value,
                       struct pcx_error **error)
{
        return (intcode_read(machine,
                             address,
                             &address,
                             error) &&
                intcode_write(machine,
                              address,
                              value,
                              error));
}

static bool
get_params(struct intcode *machine,
           int64_t opcode,
           int n_params,
           int64_t *params,
           struct pcx_error **error)
{
        opcode /= 100;

        for (int i = 0; i < n_params; i++) {
                int mode = opcode % 10;

                switch (mode) {
                case 0:
                        if (!intcode_read_indirect(machine,
                                                   machine->pc,
                                                   params + i,
                                                   error))
                                return false;
                        break;
                case 1:
                        if (!intcode_read(machine,
                                          machine->pc,
                                          params + i,
                                          error))
                                return false;
                        break;
                default:
                        pcx_set_error(error,
                                      &intcode_error_domain,
                                      INTCODE_ERROR_INVALID_ADDRESSING_MODE,
                                      "Invalid addressing mode %i at %" PRIi64,
                                      mode,
                                      machine->pc - i - 1);
                        return false;
                }

                opcode /= 10;
                machine->pc++;
        }

        return true;
}


bool
intcode_step(struct intcode *machine,
             struct pcx_error **error)
{
        int64_t instruction, opcode;
        int64_t params[MAX_PARAMS];

        if (!intcode_read(machine, machine->pc, &instruction, error))
                return false;

        opcode = instruction % 100;

        if (opcode < 0 ||
            opcode >= PCX_N_ELEMENTS(opcodes) ||
            opcodes[opcode].func == NULL) {
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_INVALID_OPCODE,
                              "Invalid opcode %" PRIi64 " at %" PRIi64,
                              opcode,
                              machine->pc);
                return false;
        }

        machine->pc++;

        if (!get_params(machine,
                        instruction,
                        opcodes[opcode].n_params,
                        params,
                        error))
                return false;

        return opcodes[opcode].func(machine, params, error);
}

bool
intcode_run(struct intcode *machine,
            struct pcx_error **error)
{
        while (!machine->stopped) {
                if (!intcode_step(machine, error))
                        return false;
        }

        return true;
}

struct intcode *
intcode_new(size_t memory_size,
            const int64_t *memory)
{
        struct intcode *machine = pcx_calloc(sizeof *machine);

        machine->memory = pcx_memdup(memory,
                                     memory_size * sizeof *machine->memory);
        machine->memory_size = memory_size;

        return machine;
}

void
intcode_free(struct intcode *machine)
{
        pcx_free(machine->memory);
        pcx_free(machine);
}
