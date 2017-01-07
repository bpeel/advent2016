use std::io;
use std::io::BufRead;

struct Direction {
    x : i32,
    y : i32,
}

impl Direction {
    fn new() -> Direction {
        Direction { x: 0, y: -1 }
    }

    fn left(&self) -> Direction {
        Direction { x: if self.x == 0 { -self.y } else { 0 },
                    y: if self.y == 0 { self.x } else { 0 } }
    }

    fn right(&self) -> Direction {
        Direction { x: if self.x == 0 { self.y } else { 0 },
                    y: if self.y == 0 { -self.x } else { 0 } }
    }
}

struct Position {
    x : i32,
    y : i32,
    direction : Direction
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            direction: Direction::new()
        }
    }

    fn step(&mut self, distance : i32) {
        self.x += self.direction.x * distance;
        self.y += self.direction.y * distance;
    }

    fn follow_instruction(&mut self, instruction : &str) {
        let mut p = instruction.chars();
        match p.next().unwrap() {
            'L' => self.direction = self.direction.left(),
            'R' => self.direction = self.direction.right(),
            _ => unreachable!()
        }
        self.step(p.as_str().parse().unwrap());
    }
}

fn main() {
    let mut pos = Position::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        for part in line.unwrap().split(", ") {
            pos.follow_instruction(part);
        }
    }

    print!("{}\n", pos.x + pos.y);
}
