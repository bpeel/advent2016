import sys
import re
from itertools import count

class Instruction:
    def __init__(self, opcode, args):
        self.opcode = opcode
        self.args = args

    def dup(self):
        return Instruction(self.opcode, self.args)

class Machine:
    def __init__(self, instructions):
        self.registers = {}
        self.instructions = [x.dup() for x in instructions]
        self.output_count = 0

    def get_arg(self, arg):
        if isinstance(arg, int):
            return arg
        return self.registers[arg]

    def cpy(self, inst):
        if isinstance(inst.args[1], str):
            self.registers[inst.args[1]] = self.get_arg(inst.args[0])
        self.pc += 1

    def add(self, inst, amount):
        if isinstance(inst.args[0], str):
            if self.pc + 3 <= len(self.instructions):
                n = self.instructions[self.pc + 1]
                nn = self.instructions[self.pc + 2]
                if (n.opcode == "dec" and
                    nn.opcode == "jnz" and
                    n.args[0] == nn.args[0] and
                    isinstance(n.args[0], str) and
                    nn.args[1] == -2):

                    amount *= self.get_arg(n.args[0])
                    self.pc += 2

                    if self.pc + 5 <= len(self.instructions):
                        nb = self.instructions[self.pc + 1]
                        nnb = self.instructions[self.pc + 2]
                        if (nb.opcode == "dec" and
                            nnb.opcode == "jnz" and
                            nb.args[0] == nnb.args[0] and
                            isinstance(nb.args[0], str) and
                            nb.args[0] != n.args[0] and
                            nnb.args[1] == -5):

                            amount *= self.get_arg(nb.args[0])
                            self.pc += 2

            self.registers[inst.args[0]] += amount
        self.pc += 1

    def inc(self, inst):
        self.add(inst, 1)

    def dec(self, inst):
        self.add(inst, -1)

    def jnz(self, inst):
        if self.get_arg(inst.args[0]) != 0:
            self.pc += self.get_arg(inst.args[1])
        else:
            self.pc += 1

    def out(self, inst):
        v = self.get_arg(inst.args[0])
        if v == (self.output_count & 1):
            self.output_count += 1
            self.pc += 1
        else:
            self.pc = len(self.instructions)
            
    opcodes = {
        "cpy" : cpy,
        "inc" : inc,
        "dec" : dec,
        "jnz" : jnz,
        "out" : out,
    }

    def execute(self, reg_a):
        self.pc = 0
        self.output_count = 0
        count = 0
        for reg in "bcd":
            self.registers[reg] = 0
        self.registers['a'] = reg_a

        while self.pc < len(self.instructions) and count < 1000000:
            inst = self.instructions[self.pc]
            Machine.opcodes[inst.opcode](self, inst)
            count += 1

        return self.output_count

def get_arg(arg):
    if arg[0].isalpha():
        return arg
    else:
        return int(arg)

def get_instruction(line):
    md = re.match(r'([a-z]{3}) ([a-d]|[+-]?[0-9]+)(?: ([a-d]|[+-]?[0-9]+))?$',
                  line)
    args = [get_arg(md.group(2))]

    if md.group(3):
        args.append(get_arg(md.group(3)))

    return Instruction(md.group(1), args)

machine = Machine([get_instruction(line) for line in sys.stdin])

for a_start in count():
    print(a_start)
    output_count = machine.execute(a_start)
    print("output_count = ", output_count)
    if output_count > 100:
        break
