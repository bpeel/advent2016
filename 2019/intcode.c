#include "intcode.h"

#include <stdbool.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>

#include "pcx-util.h"
#include "pcx-buffer.h"

#define MAX_PARAMS 3

struct pcx_error_domain
intcode_error_domain;

struct intcode {
        struct pcx_buffer memory;

        intcode_input_function input_function;
        void *input_user_data;
        intcode_output_function output_function;
        void *output_user_data;

        int64_t pc;
        int64_t ra;
        int64_t current_instruction_start;
        int64_t current_instruction;
        bool stopped;
};

struct opcode {
        int n_params;
        bool (* func)(struct intcode *machine,
                      const int64_t *params,
                      struct pcx_error **error);
};

static bool
stdin_input_cb(void *user_data,
               int64_t *value)
{
        int int_value;
        int got = fscanf(stdin, "%d", &int_value);

        if (got == 1) {
                *value = int_value;
                return true;
        } else {
                return false;
        }
}

static bool
stdout_output_cb(void *user_data,
                 int64_t value)
{
        return fprintf(stdout, "%" PRIi64 "\n", value) >= 0;
}

static bool
get_address(const struct intcode *machine,
            int addressing_mode,
            int64_t *address_in_out,
            struct pcx_error **error)
{
        switch (addressing_mode) {
        case 0:
                if (!intcode_read(machine,
                                  *address_in_out,
                                  address_in_out,
                                  error))
                        return false;
                break;
        case 2:
                if (!intcode_read(machine,
                                  *address_in_out,
                                  address_in_out,
                                  error))
                        return false;
                *address_in_out += machine->ra;
                break;
        default:
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_INVALID_ADDRESSING_MODE,
                              "Invalid addressing mode %i at %" PRIi64,
                              addressing_mode,
                              machine->current_instruction_start);
                return false;
        }

        return true;
}

static bool
store_result(struct intcode *machine,
             int addressing_mode,
             int64_t result,
             struct pcx_error **error)
{
        int64_t address = machine->pc;

        if (!get_address(machine,
                         addressing_mode,
                         &address,
                         error))
                return false;

        if (!intcode_write(machine,
                           address,
                           result,
                           error))
                return false;

        machine->pc++;

        return true;
}

#define ALU_OP(name, code)                                              \
        static bool                                                     \
        opcode_ ## name(struct intcode *machine,                        \
                        const int64_t *params,                          \
                        struct pcx_error **error)                       \
        {                                                               \
        int64_t a = params[0], b = params[1];                           \
        int write_mode = machine->current_instruction / 10000 % 10;     \
                                                                        \
        return store_result(machine, write_mode, (code), error);        \
        }

ALU_OP(add, a + b);
ALU_OP(multiply, a * b);
ALU_OP(less_than, a < b);
ALU_OP(equals, a == b);

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
        int64_t value;

        if (!machine->input_function(machine->input_user_data, &value)) {
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_IO,
                              "Error getting input");
                return false;
        }

        return store_result(machine,
                            machine->current_instruction / 100 % 10,
                            value,
                            error);
}

static bool
opcode_output(struct intcode *machine,
              const int64_t *params,
              struct pcx_error **error)
{
        if (!machine->output_function(machine->output_user_data, params[0])) {
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_IO,
                              "Error writing output");
                return false;
        }

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

static bool
opcode_relative_offset(struct intcode *machine,
                       const int64_t *params,
                       struct pcx_error **error)
{
        machine->ra += params[0];

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
        [7] = { 2, opcode_less_than },
        [8] = { 2, opcode_equals },
        [9] = { 1, opcode_relative_offset },
        [99] = { 0, opcode_stop }
};

static bool
check_address(const struct intcode *machine,
              int64_t address,
              const char *action,
              struct pcx_error **error)
{
        if (address < 0) {
                pcx_set_error(error,
                              &intcode_error_domain,
                              INTCODE_ERROR_INVALID_ADDRESS,
                              "Invalid %s %" PRIi64 " at %" PRIi64,
                              action,
                              address,
                              machine->current_instruction_start);
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

        if (address * sizeof (int64_t) >= machine->memory.length)
                *value = 0;
        else
                *value = ((int64_t *) machine->memory.data)[address];

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

        if (address * sizeof (int64_t) >= machine->memory.length) {
                size_t to_add = ((address + 1) * sizeof (int64_t) -
                                 machine->memory.length);
                pcx_buffer_ensure_size(&machine->memory,
                                       to_add + machine->memory.length);
                memset(machine->memory.data + machine->memory.length,
                       0,
                       to_add);
                machine->memory.length += to_add;
        }

        ((int64_t *) machine->memory.data)[address] = value;

        return true;
}

static bool
get_params(struct intcode *machine,
           int64_t instruction,
           int n_params,
           int64_t *params,
           struct pcx_error **error)
{
        instruction /= 100;

        for (int i = 0; i < n_params; i++) {
                int mode = instruction % 10;
                int64_t address = machine->pc;

                if (mode != 1 &&
                    !get_address(machine,
                                 mode,
                                 &address,
                                 error))
                        return false;

                if (!intcode_read(machine,
                                  address,
                                  params + i,
                                  error))
                        return false;

                instruction /= 10;
                machine->pc++;
        }

        return true;
}


bool
intcode_step(struct intcode *machine,
             struct pcx_error **error)
{
        int64_t opcode;
        int64_t params[MAX_PARAMS];

        machine->current_instruction_start = machine->pc;

        if (!intcode_read(machine,
                          machine->pc,
                          &machine->current_instruction,
                          error))
                return false;

        opcode = machine->current_instruction % 100;

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
                        machine->current_instruction,
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

        pcx_buffer_init(&machine->memory);
        pcx_buffer_append(&machine->memory,
                          memory,
                          memory_size * sizeof *memory);

        machine->input_function = stdin_input_cb;
        machine->output_function = stdout_output_cb;

        return machine;
}

void
intcode_set_input_function(struct intcode *machine,
                           intcode_input_function func,
                           void *user_data)
{
        machine->input_function = func;
        machine->input_user_data = user_data;
}

void
intcode_set_output_function(struct intcode *machine,
                            intcode_output_function func,
                            void *user_data)
{
        machine->output_function = func;
        machine->output_user_data = user_data;
}

void
intcode_free(struct intcode *machine)
{
        pcx_buffer_destroy(&machine->memory);
        pcx_free(machine);
}
