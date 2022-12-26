mod util;

use std::io::BufRead;
use util::Grid;

#[derive(Copy, Clone, Debug)]
enum Action {
    Left,
    Right,
    Forward(usize),
}

static OFFSETS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

#[derive(Clone, Debug)]
struct State<'a> {
    pos: (i32, i32),
    direction: usize,
    grid: &'a Grid,
}

impl<'a> State<'a> {
    fn new(grid: &'a Grid, pos: (i32, i32)) -> State {
        State {
            pos,
            direction: 0,
            grid,
        }
    }

    fn act(&mut self, action: Action) {
        match action {
            Action::Left => self.direction = (self.direction + 3) % 4,
            Action::Right => self.direction = (self.direction + 1) % 4,
            Action::Forward(n) => {
                for _ in 0..n {
                    let pos = self.next_pos();
                    match self.grid.get(pos).unwrap() {
                        b'.' => self.pos = pos,
                        _ => break,
                    }
                }
            },
        }
    }

    fn password(&self) -> i32 {
        (self.pos.1 + 1) * 1000 + (self.pos.0 + 1) * 4 + self.direction as i32
    }

    fn next_pos(&self) -> (i32, i32) {
        let offset = OFFSETS[self.direction];
        let next_pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        match self.grid.get(next_pos) {
            None | Some(b' ') => self.first_pos(),
            _ => next_pos,
        }
    }

    fn first_pos(&self) -> (i32, i32) {
        let mut pos = match self.direction {
            0 => (0, self.pos.1),
            1 => (self.pos.0, 0),
            2 => (self.grid.width as i32 - 1, self.pos.1),
            3 => (self.pos.0, self.grid.height as i32 - 1),
            _ => panic!("impossible direction"),
        };

        let offset = OFFSETS[self.direction];

        loop {
            if self.grid.get(pos).unwrap() != b' ' {
                break pos;
            }
            
            pos.0 += offset.0;
            pos.1 += offset.1;
        }
    }
}

fn find_start_pos(grid: &Grid) -> Option<(i32, i32)> {
    let pos = match grid.values.iter().position(|&b| b == b'.') {
        Some(p) => p,
        None => return None,
    };

    Some(((pos % grid.width) as i32, (pos / grid.width) as i32))
}
 
fn parse_password(s: &str) -> Result<Box<[Action]>, String> {
    let mut parts = Vec::<Action>::new();
    let mut num = 0;

    for ch in s.chars() {
        if let '0'..='9' = ch {
            num = (num * 10) + ch as usize - '0' as usize;
        } else {
            if num > 0 {
                parts.push(Action::Forward(num));
                num = 0;
            }

            parts.push(match ch {
                'L' => Action::Left,
                'R' => Action::Right,
                _ => return Err("unknown char in password".to_string()),
            });
        }
    }

    if num > 0 {
        parts.push(Action::Forward(num));
    }

    Ok(parts.into_boxed_slice())
}

fn read_input<I: BufRead>(input: &mut I) -> Result<(Grid, Box<[Action]>), String> {
    let grid = match Grid::load(input) {
        Err(e) => return Err(e.to_string()),
        Ok(grid) => grid,
    };

    let mut password = String::new();

    if let Err(e) = input.read_line(&mut password) {
        return Err(e.to_string());
    };

    let len = password.trim_end().len();
    password.truncate(len);

    Ok((grid, parse_password(password.trim_end())?))
}

fn main() -> std::process::ExitCode {
    let (grid, password) = match read_input(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(v) => v,
    };

    let start_pos = match find_start_pos(&grid) {
        None => {
            eprintln!("no start pos");
            return std::process::ExitCode::FAILURE;
        },
        Some(p) => p,
    };

    let mut state = State::new(&grid, start_pos);

    for &action in password.iter() {
        state.act(action)
    }

    println!("{}", grid);
    println!("{:?}", start_pos);
    println!("{:?}", password);
    println!("{:?}", state);

    println!("part 1: {}", state.password());

    std::process::ExitCode::SUCCESS
}
