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

struct Ferry2 {
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ferry2 {
    fn new() -> Ferry2 {
        Ferry2 {
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: -1,
        }
    }

    fn rotate(&mut self, amount: i32) {
        let (x, y) = match amount {
            0 => (self.waypoint_x, self.waypoint_y),
            1 => (-self.waypoint_y, self.waypoint_x),
            2 => (-self.waypoint_x, -self.waypoint_y),
            3 => (self.waypoint_y, -self.waypoint_x),
            _ => unreachable!(),
        };

        self.waypoint_x = x;
        self.waypoint_y = y;
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::North => self.waypoint_y -= instruction.distance,
            Action::East => self.waypoint_x += instruction.distance,
            Action::South => self.waypoint_y += instruction.distance,
            Action::West => self.waypoint_y -= instruction.distance,
            Action::Left => {
                self.rotate((4 - instruction.distance / 90 % 4) % 4);
            },
            Action::Right => self.rotate(instruction.distance / 90 % 4),
            Action::Forward => {
                self.x += instruction.distance * self.waypoint_x;
                self.y += instruction.distance * self.waypoint_y;
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

    let mut ferry = Ferry2::new();

    for instruction in instructions.iter() {
        ferry.apply_instruction(instruction);
    }

    println!("part 2: {}", ferry.x.abs() + ferry.y.abs());

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    const POSITIONS: [(i32, i32); 4] = [
            (1, -2),
            (2, 1),
            (-1, 2),
            (-2, -1),
    ];

    #[test]
    fn rotate_left() {
        for i in 1..=3 {
            let mut ferry = Ferry2::new();
            ferry.waypoint_x = POSITIONS[0].0;
            ferry.waypoint_y = POSITIONS[0].1;
            ferry.apply_instruction(&Instruction {
                action: Action::Left,
                distance: i * 90,
            });
            assert_eq!(
                (ferry.waypoint_x, ferry.waypoint_y),
                POSITIONS[((4 - i) % 4) as usize]
            );
        }
    }

    #[test]
    fn rotate_right() {
        for i in 1..=3 {
            let mut ferry = Ferry2::new();
            ferry.waypoint_x = POSITIONS[0].0;
            ferry.waypoint_y = POSITIONS[0].1;
            ferry.apply_instruction(&Instruction {
                action: Action::Right,
                distance: i * 90,
            });
            assert_eq!(
                (ferry.waypoint_x, ferry.waypoint_y),
                POSITIONS[i as usize]
            );
        }
    }
}
