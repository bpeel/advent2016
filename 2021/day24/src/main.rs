use std::str::FromStr;
use std::fmt;
use std::process::ExitCode;

const N_REGISTERS: usize = 4;
const MONAD_LENGTH: usize = 14;

#[derive(Eq, PartialEq)]
enum OpArg {
    Register(u8),
    Literal(i64),
}

#[derive(Clone, Copy, Eq, PartialEq)]
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

fn count_inps(program: &[Op]) -> usize {
    program.iter().filter(|op| matches!(op.opcode, Opcode::Inp)).count()
}

enum Source {
    Inputs(u16, i64, i64),
    Constant(i64),
}

impl Source {
    fn combine(&self, opcode: ArithmeticOpcode, b: &Source) -> Source {
        match (self, b) {
            (Source::Constant(a), Source::Constant(b)) => match opcode {
                ArithmeticOpcode::Add => Source::Constant(a + b),
                ArithmeticOpcode::Mul => Source::Constant(a * b),
                ArithmeticOpcode::Div => Source::Constant(a / b),
                ArithmeticOpcode::Mod => Source::Constant(a % b),
                ArithmeticOpcode::Eql => Source::Constant((a == b) as i64),
            },
            (&Source::Inputs(inputs, min, max), &Source::Constant(b)) => {
                match opcode {
                    ArithmeticOpcode::Add => {
                        Source::Inputs(
                            inputs,
                            min.saturating_add(b),
                            max.saturating_add(b),
                        )
                    },
                    ArithmeticOpcode::Mul => {
                        if b == 0 {
                            Source::Constant(0)
                        } else {
                            Source::Inputs(
                                inputs,
                                min.saturating_mul(b),
                                max.saturating_mul(b),
                            )
                        }
                    },
                    ArithmeticOpcode::Div => {
                        Source::Inputs(inputs, min / b, max / b)
                    },
                    ArithmeticOpcode::Mod => {
                        let max = std::cmp::min(max, b - 1);
                        let min = std::cmp::min(min, max);
                        Source::Inputs(inputs, min, max)
                    },
                    ArithmeticOpcode::Eql => {
                        if b < min || b > max {
                            Source::Constant(0)
                        } else {
                            Source::Inputs(inputs, 0, 1)
                        }
                    },
                }
            },
            (Source::Constant(_), Source::Inputs(..)) => match opcode {
                ArithmeticOpcode::Add |
                ArithmeticOpcode::Mul |
                ArithmeticOpcode::Eql => {
                    b.combine(opcode, self)
                },
                ArithmeticOpcode::Div => {
                    // fixme
                    unreachable!()
                },
                ArithmeticOpcode::Mod => {
                    // fixme
                    unreachable!()
                },
            },
            (
                &Source::Inputs(a, a_min, a_max),
                &Source::Inputs(b, b_min, b_max),
            ) => {
                let (min, max) = match opcode {
                    ArithmeticOpcode::Add => (a_min + b_min, a_max + b_max),
                    ArithmeticOpcode::Mul => {
                        let combos = [
                            a_min.saturating_mul(b_min),
                            a_min.saturating_mul(b_max),
                            a_max.saturating_mul(b_min),
                            a_max.saturating_mul(b_max),
                        ];
                        let min = combos.iter().map(|&a| a).min().unwrap();
                        (
                            min,
                            combos.into_iter().max().unwrap(),
                        )
                    },
                    ArithmeticOpcode::Eql => (0, 1),
                    ArithmeticOpcode::Div => {
                        // fixme
                        unreachable!()
                    },
                    ArithmeticOpcode::Mod => {
                        // fixme
                        unreachable!()
                    },
                };

                Source::Inputs(a | b, min, max)
            }
        }
    }
}

fn source_for_register(program: &[Op], reg: u8) -> Source {
    // Find the last instruction that writes to the register
    for (instruction_num, op) in program.iter().enumerate().rev() {
        if op.a == reg {
            return source_for_op(&program[0..instruction_num], op)
        }
    }

    Source::Constant(0)
}

fn source_for_op(program: &[Op], op: &Op) -> Source {
    match &op.opcode {
        Opcode::Inp => {
            Source::Inputs(
                1u16 << (count_inps(program) as u16),
                1,
                9,
            )
        },
        Opcode::Arithmetic(opcode, arg) => {
            let b_input = match arg {
                OpArg::Literal(value) => {
                    if *opcode == ArithmeticOpcode::Mul && *value == 0 {
                        return Source::Constant(0);
                    } else {
                        Source::Constant(*value)
                    }
                },
                OpArg::Register(b) => source_for_register(program, *b),
            };

            source_for_register(program, op.a).combine(*opcode, &b_input)
        }
    }
}

fn run_args(mod_adds: &[i64], offsets: &[i64]) {
    let mut args = std::env::args();

    if args.next().is_none() {
        return;
    }

    for arg in args {
        let mut mod_adds = mod_adds.iter();
        let mut offsets = offsets.iter();
        let mut wip = 0;

        for ch in arg.chars() {
            let input = ch as i64 - '0' as i64;
            let Some(mod_add) = mod_adds.next()
            else { break; };
            let Some(offset) = offsets.next()
            else { break; };

            let old_wip = wip;

            if *mod_add < 0 {
                wip /= 26;
            }

            let need_to_match = old_wip % 26 + mod_add;

            if input != need_to_match {
                wip = wip * 26 + input + offset;
            }

            println!(
                "input: {:<3} mod_add: {:<3} offset: {:<3}\
                 wip: {:<10} need to match: {}{:<4}{}",
                input,
                mod_add,
                offset,
                wip,
                if *mod_add < 0 { '*' } else { ' ' },
                need_to_match,
                if input == need_to_match { '👍' } else { ' ' },
            );
        }
    }
}

fn part1_interpret(program: &[Op]) -> Result<Option<Monad>, SearchError> {
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

fn part1_template(mod_adds: &[i64], offsets: &[i64]) -> Option<Monad> {
    let mut monad = Monad::highest();

    loop {
        let mut wip = 0;

        for digit in 0..MONAD_LENGTH {
            let mod_add = mod_adds[digit];
            let old_wip = wip;

            if mod_add < 0 {
                wip /= 26;
            }

            let input = monad.digits[digit] as i64;

            if input != old_wip % 26 + mod_add {
                wip = wip * 26 + input + offsets[digit];
            }
        }

        if wip == 0 {
            break Some(monad);
        }

        if !monad.previous() {
            break None;
        }
    }
}

fn part1(program: &[Op]) -> Result<Option<Monad>, SearchError> {
    if let Some((mod_adds, offsets)) = match_template(program) {
        println!("template matched");
        run_args(&mod_adds, &offsets);
        Ok(part1_template(&mod_adds, &offsets))
    } else {
        part1_interpret(program)
    }
}

fn match_instructions<'a, T: Iterator<Item = &'a Op>>(
    ops: &mut T,
    template: &[(ArithmeticOpcode, u8, OpArg)],
) -> bool {
    for (opcode, dest, arg) in template.iter() {
        let Some(Op { a, opcode: Opcode::Arithmetic(real_opcode, real_arg) }) =
            ops.next()
        else { return false; };

        if opcode != real_opcode || real_arg != arg || dest != a {
            return false;
        }
    }

    true
}

fn match_template(program: &[Op]) -> Option<(Vec<i64>, Vec<i64>)> {
    let mut ops = program.iter();
    let mut mod_adds = Vec::new();
    let mut offsets = Vec::new();

    for _ in 0..MONAD_LENGTH {
        let Some(Op { a, opcode: Opcode::Inp }) = ops.next()
        else { return None; };

        if *a != 0 {
            return None;
        }

        if !match_instructions(
            &mut ops,
            &[
                (ArithmeticOpcode::Mul, 1, OpArg::Literal(0)),
                (ArithmeticOpcode::Add, 1, OpArg::Register(3)),
                (ArithmeticOpcode::Mod, 1, OpArg::Literal(26)),
            ]
        ) {
            return None;
        }

        let mod_add_is_negative = {
            let Some(Op {
                a,
                opcode: Opcode::Arithmetic(
                    ArithmeticOpcode::Div,
                    OpArg::Literal(divisor),
                ),
            }) = ops.next()
            else { return None; };

            if *a != 3 {
                return None;
            }

            if *divisor == 26 {
                true
            } else if *divisor == 1 {
                false
            } else {
                return None;
            }
        };

        let Some(Op {
            a,
            opcode: Opcode::Arithmetic(
                ArithmeticOpcode::Add,
                OpArg::Literal(mod_add),
            ),
        }) = ops.next()
        else { return None; };

        if *a != 1 {
            return None;
        }

        if (*mod_add < 0) != mod_add_is_negative {
            return None;
        }

        mod_adds.push(*mod_add);

        if !match_instructions(
            &mut ops,
            &[
                (ArithmeticOpcode::Eql, 1, OpArg::Register(0)),
                (ArithmeticOpcode::Eql, 1, OpArg::Literal(0)),
                (ArithmeticOpcode::Mul, 2, OpArg::Literal(0)),
                (ArithmeticOpcode::Add, 2, OpArg::Literal(25)),
                (ArithmeticOpcode::Mul, 2, OpArg::Register(1)),
                (ArithmeticOpcode::Add, 2, OpArg::Literal(1)),
                (ArithmeticOpcode::Mul, 3, OpArg::Register(2)),
                (ArithmeticOpcode::Mul, 2, OpArg::Literal(0)),
                (ArithmeticOpcode::Add, 2, OpArg::Register(0)),
            ]
        ) {
            return None;
        }

        let Some(Op {
            a,
            opcode: Opcode::Arithmetic(
                ArithmeticOpcode::Add,
                OpArg::Literal(offset),
            ),
        }) = ops.next()
        else { return None; };

        if *a != 2 {
            return None;
        }

        offsets.push(*offset);

        if !match_instructions(
            &mut ops,
            &[
                (ArithmeticOpcode::Mul, 2, OpArg::Register(1)),
                (ArithmeticOpcode::Add, 3, OpArg::Register(2)),
            ]
        ) {
            return None;
        }
    }

    if ops.next().is_none() {
        Some((mod_adds, offsets))
    } else {
        None
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

    match source_for_register(&program, N_REGISTERS as u8 - 1) {
        Source::Constant(value) => println!("z is constant: {}", value),
        Source::Inputs(inputs, min, max) => {
            println!(
                "z is derived from inputs 0x{:x} and has range {}<=x<={}",
                inputs,
                min,
                max,
            );
        },
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
