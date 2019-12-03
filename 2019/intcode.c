#include "intcode.h"

#include <stdbool.h>
#include <stdlib.h>
#include <inttypes.h>

#include "pcx-util.h"

struct pcx_error_domain
intcode_error_domain;

struct intcode {
        size_t memory_size;
        int64_t *memory;
        int64_t pc;
        bool stopped;
};

typedef bool
(* opcode_func)(struct intcode *machine,
                struct pcx_error **error);

static bool
start_alu(struct intcode *machine,
          int64_t *a,
          int64_t *b,
          struct pcx_error **error)
{
        if (!intcode_read_indirect(machine,
                                   machine->pc,
                                   a,
                                   error) ||
            !intcode_read_indirect(machine,
                                   machine->pc + 1,
                                   b,
                                   error))
                return false;

        machine->pc += 2;

        return true;
}

static bool
end_alu(struct intcode *machine,
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

#define ALU_OP(name, code)                              \
        static bool                                     \
        opcode_ ## name(struct intcode *machine,        \
                        struct pcx_error **error)       \
        {                                               \
                int64_t a, b;                           \
                                                        \
                if (!start_alu(machine, &a, &b, error)) \
                        return false;                   \
                                                        \
                return end_alu(machine, (code), error); \
        }

ALU_OP(add, a + b);
ALU_OP(multiply, a * b);

#undef ALU_OP

static bool
opcode_stop(struct intcode *machine,
            struct pcx_error **error)
{
        machine->stopped = true;

        return true;
}

static const opcode_func
opcodes[] = {
        [1] = opcode_add,
        [2] = opcode_multiply,
        [99] = opcode_stop,
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

bool
intcode_step(struct intcode *machine,
             struct pcx_error **error)
{
        int64_t opcode;

        if (!intcode_read(machine, machine->pc, &opcode, error))
                return false;

        if (opcode < 0 ||
            opcode >= PCX_N_ELEMENTS(opcodes) ||
            opcodes[opcode] == NULL) {
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_INVALID_OPCODE,
                              "Invalid opcode %" PRIi64 " at %" PRIi64,
                              opcode,
                              machine->pc);
                return false;
        }

        machine->pc++;

        return opcodes[opcode](machine, error);
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
