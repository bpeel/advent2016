use std::io;
use std::io::BufRead;

#[derive(Copy, Clone)]
enum Rotation {
    Left,
    Right
}

#[derive(Copy, Clone)]
struct Instruction {
    rotation : Rotation,
    distance : u32
}

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

#[derive(Copy, Clone)]
struct Position {
    x : i32,
    y : i32,
}

struct Person {
    part : u8,
    history : Vec<Position>,
    position : Position,
    direction : Direction
}

impl Person {
    fn new(part: u8) -> Person {
        Person {
            part: part,
            history: Vec::<Position>::new(),
            position: Position { x: 0, y: 0 },
            direction: Direction::new()
        }
    }

    fn step(&mut self) {
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
    }

    fn follow_instruction(&mut self, instruction : &Instruction) -> bool {
        match instruction.rotation {
            Rotation::Left => self.direction = self.direction.left(),
            Rotation::Right => self.direction = self.direction.right(),
        }

        for _ in 0..instruction.distance {
            self.step();
            if self.part == 1 {
                for position in &self.history {
                    if position.x == self.position.x &&
                        position.y == self.position.y {
                            return true;
                        }
                }
                self.history.push(self.position);
            }
        }
        return false;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut instructions = Vec::<Instruction>::new();

    for line in stdin.lock().lines() {
        for part in line.unwrap().split(", ") {
            let mut p = part.chars();
            let dir = match p.next().unwrap() {
                'L' => Rotation::Left,
                'R' => Rotation::Right,
                _ => unreachable!()
            };

            let distance : u32 = p.as_str().parse().unwrap();

            instructions.push(Instruction { rotation: dir,
                                            distance: distance });
        }
    }

    for part in 0..2 {
        let mut person = Person::new(part);
        for instruction in &instructions {
            if person.follow_instruction(&instruction) {
                break;
            }
        }
        print!("Part {}: {}\n",
               part + 1,
               person.position.x + person.position.y);
    }
}
