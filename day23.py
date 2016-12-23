import sys
import re

class Instruction:
    def __init__(self, opcode, args):
        self.opcode = opcode
        self.args = args

class Machine:
    def __init__(self, instructions):
        self.registers = {}
        self.instructions = instructions

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

    def tgl(self, inst):
        inst_num = self.pc + self.get_arg(inst.args[0])
        if inst_num >= 0 and inst_num < len(self.instructions):
            other = self.instructions[inst_num]
            if len(other.args) == 1:
                if other.opcode == "inc":
                    other.opcode = "dec"
                else:
                    other.opcode = "inc"
            else:
                if other.opcode == "jnz":
                    other.opcode = "cpy"
                else:
                    other.opcode = "jnz"
        self.pc += 1
                    
    opcodes = {
        "cpy" : cpy,
        "inc" : inc,
        "dec" : dec,
        "jnz" : jnz,
        "tgl" : tgl
    }

    def execute(self):
        self.pc = 0
        for reg in "bcd":
            self.registers[reg] = 0
        self.registers['a'] = 7

        while self.pc < len(self.instructions):
            inst = self.instructions[self.pc]
            Machine.opcodes[inst.opcode](self, inst)

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
machine.execute()

print("Part 1:", machine.registers['a'])
