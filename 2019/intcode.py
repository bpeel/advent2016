#!/usr/bin/env python3

class ExecutionException(Exception):
    pass

class InvalidOpcode(ExecutionException):
    def __init__(self, pc, opcode):
        super().__init__("Invalid opcode {} at {}".format(opcode, pc))
        self.pc = pc
        self.opcode = opcode

class InvalidAddress(ExecutionException):
    def __init__(self, msg, addr):
        super().__init__(msg)
        self.addr = addr

class InvalidRead(InvalidAddress):
    def __init__(self, addr):
        super().__init__("Invalid read from {}".format(addr, addr),
                         addr)

class InvalidWrite(InvalidAddress):
    def __init__(self, addr):
        super().__init__("Invalid write to {}".format(addr, addr),
                         addr)

class Machine:
    def __init__(self, memory):
        self.memory = list(memory)
        self.pc = 0
        self._stopped = False

    def step(self):
        opcode = self.read_memory(self.pc)

        try:
            func = OPCODES[opcode]
        except KeyError:
            raise InvalidOpcode(self.pc, opcode)

        self.pc += 1

        func(self)

    def _stop(self):
        self._stopped = True

    def run(self):
        while not self._stopped:
            self.step()

    def read_memory(self, pos):
        if pos < 0 or pos >= len(self.memory):
            raise InvalidRead(pos)

        return self.memory[pos]

    def write_memory(self, pos, value):
        if pos < 0 or pos >= len(self.memory):
            raise InvalidWrite(pos)

        self.memory[pos] = value

    def read_indirect(self, pos):
        return self.read_memory(self.read_memory(pos))

    def write_indirect(self, pos, value):
        self.write_memory(self.read_memory(pos), value)

def _alu_op(n_args, func):
    def run_func(self):
        args = (self.read_indirect(self.pc + i) for i in range(n_args))
        self.write_indirect(self.pc + n_args, func(*args))
        self.pc += n_args + 1

    return run_func

OPCODES = {
    1: _alu_op(2, lambda a, b: a + b),
    2: _alu_op(2, lambda a, b: a * b),
    99: Machine._stop
}
