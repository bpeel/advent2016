use std::str::FromStr;
use std::fmt;
use std::process::ExitCode;

const N_REGISTERS: usize = 4;
const MONAD_LENGTH: usize = 14;

enum OpArg {
    Register(u8),
    Literal(i64),
}

#[derive(Clone, Copy)]
enum ArithmeticOpcode {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

enum Opcode {
    Inp,
    Arithmetic(ArithmeticOpcode, OpArg),
}

struct Op {
    a: u8,
    opcode: Opcode,
}

fn parse_register(s: &str) -> Result<u8, String> {
    let mut chars = s.chars();

    let Some(ch) = chars.next()
    else { return Err("Empty register name".to_string()); };

    if ch < 'w' || ch > ('w' as u8 + N_REGISTERS as u8) as char
        || chars.next().is_some()
    {
        return Err(format!("Invalid register name: {}", s));
    }

    Ok(ch as u8 - 'w' as u8)
}

impl FromStr for OpArg {
    type Err = String;

    fn from_str(s: &str) -> Result<OpArg, String> {
        match s.chars().next() {
            Some(digit) if digit.is_ascii_alphabetic() => {
                Ok(OpArg::Register(parse_register(s)?))
            },
            _ => {
                s.parse::<i64>()
                    .map(|v| OpArg::Literal(v))
                    .map_err(|e| e.to_string())
            },
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Op, String> {
        let Some((op_name, s)) = s.split_once(' ')
        else { return Err("Missing space".to_string()); };

        let (a, tail) = match s.split_once(' ') {
            Some((a, tail)) => (parse_register(a)?, tail),
            None => (parse_register(s)?, ""),
        };

        if op_name == "inp" {
            if !tail.is_empty() {
                return Err("Too many arguments to inp".to_string());
            }

            return Ok(Op { a, opcode: Opcode::Inp });
        }

        let b = tail.parse::<OpArg>()?;

        let opcode = match op_name {
            "add" => ArithmeticOpcode::Add,
            "mul" => ArithmeticOpcode::Mul,
            "div" => ArithmeticOpcode::Div,
            "mod" => ArithmeticOpcode::Mod,
            "eql" => ArithmeticOpcode::Eql,
            _ => return Err(format!("Unknown op: {}", op_name)),
        };

        Ok(Op { a, opcode: Opcode::Arithmetic(opcode, b) })
    }
}

enum MachineError {
    EndOfInput,
    ArithmeticError,
}

impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachineError::EndOfInput => write!(f, "End of input reached"),
            MachineError::ArithmeticError => write!(f, "Arithmetic error"),
        }
    }
}

struct Machine<'a> {
    registers: [i64; N_REGISTERS],
    input: &'a [u8],
    input_pos: usize,
}

impl<'a> Machine<'a> {
    fn new(input: &'a [u8]) -> Machine<'a> {
        Machine {
            registers: [0i64; N_REGISTERS],
            input,
            input_pos: 0,
        }
    }

    fn get_input(&mut self) -> Result<i64, MachineError> {
        match self.input.get(self.input_pos) {
            Some(&value) => {
                self.input_pos += 1;
                Ok(value as i64)
            },
            None => Err(MachineError::EndOfInput),
        }
    }

    fn evaluate_arithmetic(
        &self,
        op: ArithmeticOpcode,
        a: i64,
        b: &OpArg,
    ) -> Result<i64, MachineError> {
        let b = match b {
            OpArg::Literal(value) => *value,
            OpArg::Register(reg) => self.registers[*reg as usize],
        };

        let result = match op {
            ArithmeticOpcode::Add => a.checked_add(b),
            ArithmeticOpcode::Mul => a.checked_mul(b),
            ArithmeticOpcode::Div => a.checked_div(b),
            ArithmeticOpcode::Mod => a.checked_rem(b),
            ArithmeticOpcode::Eql => Some((a == b) as i64),
        };

        result.ok_or(MachineError::ArithmeticError)
    }

    fn execute_op(&mut self, op: &Op) -> Result<(), MachineError> {
        let regs = &self.registers;

        let result = match op.opcode {
            Opcode::Inp => self.get_input()?,
            Opcode::Arithmetic(opcode, ref b) => {
                self.evaluate_arithmetic(opcode, regs[op.a as usize], b)?
            },
        };

        self.registers[op.a as usize] = result;

        Ok(())
    }
}

struct Monad {
    digits: [u8; MONAD_LENGTH],
}

impl Monad {
    fn highest() -> Monad {
        Monad {
            digits: [9; MONAD_LENGTH],
        }
    }

    fn previous(&mut self) -> bool {
        for digit in self.digits.iter_mut().rev() {
            if *digit > 1 {
                *digit -= 1;
                return true;
            } else {
                *digit = 9;
            }
        }

        false
    }
}

impl fmt::Display for Monad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.digits.iter() {
            write!(f, "{}", (digit + '0' as u8) as char)?;
        }

        Ok(())
    }
}

struct SearchError {
    line_num: usize,
    monad: Monad,
    machine_error: MachineError,
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "line {}: monad {}: {}",
            self.line_num,
            self.monad,
            self.machine_error,
        )
    }
}

fn part1(program: &[Op]) -> Result<Option<Monad>, SearchError> {
    let mut monad = Monad::highest();

    loop {
        let mut machine = Machine::new(&monad.digits);

        for (op_num, op) in program.iter().enumerate() {
            if let Err(machine_error) = machine.execute_op(op) {
                return Err(SearchError {
                    line_num: op_num + 1,
                    monad,
                    machine_error,
                });
            }
        }

        if *machine.registers.last().unwrap() == 0 {
            break Ok(Some(monad));
        }

        if !monad.previous() {
            break Ok(None);
        }
    }
}

fn main() -> ExitCode {
    let mut program = Vec::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            }
            Ok(line) => line,
        };

        program.push(match line.parse::<Op>() {
            Err(e) => {
                eprintln!("line {}: {}", line_num + 1, e);
                return ExitCode::FAILURE;
            },
            Ok(op) => op,
        });
    }

    match part1(&program) {
        Ok(Some(monad)) => println!("part 1: {}", monad),
        Ok(None) => println!("no valid monad found"),
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    }

    ExitCode::SUCCESS
}
