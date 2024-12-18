use std::process::ExitCode;
use std::sync::LazyLock;
use std::str::FromStr;
use regex::Regex;
use std::fmt;

struct Computer {
    registers: [u64; 3],
    program: Vec<u8>,
    ip: usize,
    output: Vec<u8>,
}

enum Error {
    EndOfProgram,
    InvalidComboValue(u8),
    UnknownOpcode(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EndOfProgram => write!(f, "end of program encountered"),
            Error::InvalidComboValue(value) => {
                write!(f, "invalid combo value encountered: {}", value)
            },
            Error::UnknownOpcode(value) => {
                write!(f, "unknown opcode encountered: {}", value)
            },
        }
    }
}

impl Computer {
    fn new(initial_state: InitialState) -> Computer {
        Computer {
            registers: initial_state.registers,
            program: initial_state.program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn read_program(&mut self) -> Result<u8, Error> {
        match self.program.get(self.ip) {
            Some(&value) => {
                self.ip += 1;
                Ok(value)
            },
            None => Err(Error::EndOfProgram),
        }
    }

    fn read_combo(&mut self) -> Result<u64, Error> {
        match self.read_program()? {
            literal @ 0..=3 => Ok(literal as u64),
            register @ 4..=6 => Ok(self.registers[register as usize - 4]),
            other => Err(Error::InvalidComboValue(other)),
        }
    }

    fn read_literal(&mut self) -> Result<u64, Error> {
        self.read_program().map(|value| value as u64)
    }

    fn adv(&mut self) -> Result<(), Error> {
        let denominator = self.read_combo()?;
        self.registers[0] /= 1 << denominator;
        Ok(())
    }

    fn bxl(&mut self) -> Result<(), Error> {
        self.registers[1] ^= self.read_literal()?;
        Ok(())
    }

    fn bst(&mut self) -> Result<(), Error> {
        self.registers[1] = self.read_combo()? & 7;
        Ok(())
    }

    fn jnz(&mut self) -> Result<(), Error> {
        let target = self.read_literal()?;

        if self.registers[0] != 0 {
            self.ip = target as usize;
        }

        Ok(())
    }

    fn bxc(&mut self) -> Result<(), Error> {
        let _unused_operand = self.read_program()?;
        self.registers[1] ^= self.registers[2];
        Ok(())
    }

    fn out(&mut self) -> Result<(), Error> {
        let value = (self.read_combo()? & 7) as u8;
        self.output.push(value);
        Ok(())
    }

    fn bdv(&mut self) -> Result<(), Error> {
        let denominator = self.read_combo()?;
        self.registers[1] = self.registers[0] / (1 << denominator);
        Ok(())
    }

    fn cdv(&mut self) -> Result<(), Error> {
        let denominator = self.read_combo()?;
        self.registers[2] = self.registers[0] / (1 << denominator);
        Ok(())
    }

    fn step(&mut self) -> Result<(), Error> {
        let opcode = self.read_program()?;

        match opcode {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => Err(Error::UnknownOpcode(opcode)),
        }
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.step() {
                Ok(()) => (),
                Err(Error::EndOfProgram) => break Ok(()),
                Err(e) => break Err(e),
            }
        }
    }
}

static INITIAL_STATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"Register A: (\d+)", "\n",
        r"Register B: (\d+)", "\n",
        r"Register C: (\d+)", "\n",
        "\n",
        r"Program: (\d+(?:,\d)*)\s*$"
    )).unwrap()
});

#[derive(Debug, Clone)]
struct InitialState {
    registers: [u64; 3],
    program: Vec<u8>,
}

impl FromStr for InitialState {
    type Err = String;

    fn from_str(s: &str) -> Result<InitialState, String> {
        let Some(captures) = INITIAL_STATE_RE.captures(s)
        else {
            return Err("invalid initial state description".to_string());
        };

        let mut registers = [0u64; 3];

        for (i, register) in registers.iter_mut().enumerate() {
            let Ok(value) = captures[i + 1].parse::<u64>()
            else {
                return Err(format!(
                    "invalid value for register {}",
                    (i as u8 + b'A') as char,
                ));
            };

            *register = value;
        }

        let mut program = Vec::new();

        for instruction in captures[4].split(',') {
            let Ok(value) = instruction.parse::<u8>()
            else {
                return Err(format!(
                    "invalid value in program: {}",
                    instruction,
                ));
            };

            if value >= 8 {
                return Err(format!(
                    "opcode is out of range: {}",
                    instruction,
                ));
            }

            program.push(value);
        }

        Ok(InitialState {
            registers,
            program,
        })
    }
}

fn read_initial_state() -> Result<InitialState, String> {
    match std::io::read_to_string(std::io::stdin().lock()) {
        Ok(source) => source.parse::<InitialState>(),
        Err(e) => Err(e.to_string()),
    }
}

fn main() -> ExitCode {
    let initial_state = match read_initial_state() {
        Ok(is) => is,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let mut computer = Computer::new(initial_state.clone());

    if let Err(e) = computer.run() {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    print!("Part 1: ");

    for (i, &value) in computer.output.iter().enumerate() {
        if i > 0 {
            print!(",");
        }

        print!("{}", value);
    }

    println!();

    for a in 0.. {
        computer.registers = initial_state.registers;
        computer.registers[0] = a;
        computer.output.clear();
        computer.ip = 0;

        if let Err(e) = computer.run() {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }

        if computer.output == computer.program {
            println!("Part 2: {}", a);
            break;
        }
    }

    ExitCode::SUCCESS
}
