use std::str::FromStr;
use std::process::ExitCode;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Action {
    North,
    East,
    South,
    West,
    Forward,
    Left,
    Right,
}

struct Instruction {
    action: Action,
    distance: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        let mut chars = s.chars();

        let action = match chars.next() {
            Some('N') => Action::North,
            Some('E') => Action::East,
            Some('S') => Action::South,
            Some('W') => Action::West,
            Some('F') => Action::Forward,
            Some('L') => Action::Left,
            Some('R') => Action::Right,
            _ => return Err(()),
        };

        let Ok(distance) = chars.as_str().parse::<i32>()
        else {
            return Err(());
        };

        if action == Action::Left || action == Action::Right {
            if distance % 90 != 0 {
                return Err(());
            }
        }

        Ok(Instruction { action, distance })
    }
}

struct Ferry {
    x: i32,
    y: i32,
    // 0 = East, 1 = South, 2 = West, 3 = North
    direction: i32,
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            x: 0,
            y: 0,
            direction: 0,
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::North => self.y -= instruction.distance,
            Action::East => self.x += instruction.distance,
            Action::South => self.y += instruction.distance,
            Action::West => self.y -= instruction.distance,
            Action::Left => {
                self.direction = (self.direction
                                  + (4 - instruction.distance / 90 % 4) % 4)
                    % 4;
            },
            Action::Right => {
                self.direction = (self.direction
                                  + instruction.distance / 90)
                    % 4;
            },
            Action::Forward => {
                match self.direction {
                    0 => self.x += instruction.distance,
                    1 => self.y += instruction.distance,
                    2 => self.x -= instruction.distance,
                    3 => self.y -= instruction.distance,
                    _ => unreachable!(),
                }
            },
        }
    }
}

fn read_instructions() -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(e.to_string()),
        };

        let instruction = match line.parse::<Instruction>() {
            Ok(instruction) => instruction,
            Err(_) => return Err(
                format!("line {}: invalid action", line_num + 1)
            ),
        };

        instructions.push(instruction);
    }

    Ok(instructions)
}

fn main() -> ExitCode {
    let instructions = match read_instructions() {
        Ok(instructions) => instructions,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let mut ferry = Ferry::new();

    for instruction in instructions.iter() {
        ferry.apply_instruction(instruction);
    }

    println!("part 1: {}", ferry.x.abs() + ferry.y.abs());

    ExitCode::SUCCESS
}
