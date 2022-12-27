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

const N_FACES: usize = 6;

struct FaceLink {
    next_face: usize,
    rotation: usize,
}

struct Face {
    links: [FaceLink; 4],
}

#[derive(Debug, Clone, Copy)]
struct FacePos {
    top_left: (i32, i32),
    rotation: usize,
}

static FACES: [Face; N_FACES] = [
    Face {
        links: [
            FaceLink { next_face: 5, rotation: 1 },
            FaceLink { next_face: 2, rotation: 0 },
            FaceLink { next_face: 3, rotation: 2 },
            FaceLink { next_face: 1, rotation: 0 },
        ],
    },
    Face {
        links: [
            FaceLink { next_face: 5, rotation: 2 },
            FaceLink { next_face: 0, rotation: 0 },
            FaceLink { next_face: 3, rotation: 1 },
            FaceLink { next_face: 4, rotation: 0 },
        ],
    },
    Face {
        links: [
            FaceLink { next_face: 5, rotation: 0 },
            FaceLink { next_face: 4, rotation: 0 },
            FaceLink { next_face: 3, rotation: 3 },
            FaceLink { next_face: 0, rotation: 0 },
        ],
    },
    Face {
        links: [
            FaceLink { next_face: 4, rotation: 0 },
            FaceLink { next_face: 1, rotation: 3 },
            FaceLink { next_face: 0, rotation: 2 },
            FaceLink { next_face: 2, rotation: 1 },
        ],
    },
    Face {
        links: [
            FaceLink { next_face: 5, rotation: 3 },
            FaceLink { next_face: 1, rotation: 0 },
            FaceLink { next_face: 3, rotation: 0 },
            FaceLink { next_face: 2, rotation: 0 },
        ],
    },
    Face {
        links: [
            FaceLink { next_face: 1, rotation: 2 },
            FaceLink { next_face: 4, rotation: 1 },
            FaceLink { next_face: 2, rotation: 0 },
            FaceLink { next_face: 0, rotation: 3 },
        ],
    },
];

#[derive(Clone, Debug)]
struct State<'a> {
    pos: (i32, i32),
    direction: usize,
    grid: &'a Grid,
    cube: bool,
    face_length: usize,
    face_map: [FacePos; N_FACES],
    faces_found: u8,
}

impl<'a> State<'a> {
    fn new(grid: &'a Grid, cube: bool) -> Result<State, String> {
        let face_length = if std::cmp::min(grid.width, grid.height) > 50 {
            50
        } else {
            4
        };

        let start_pos = match find_start_pos(grid) {
            None => return Err("no start pos".to_string()),
            Some(p) => p,
        };

        let mut state = State {
            pos: start_pos,
            direction: 0,
            grid,
            cube,
            face_length,
            face_map: [FacePos { top_left: (0, 0), rotation: 0 }; N_FACES],
            faces_found: 0,
        };

        state.add_face(0, start_pos, 0);

        if state.faces_found != (1u8 << 6) - 1 {
            return Err("couldn’t find all faces".to_string());
        }

        Ok(state)
    }

    fn add_face(&mut self, face: usize, pos: (i32, i32), rotation: usize) {
        if self.faces_found & (1u8 << face as u8) != 0 {
            return;
        }

        self.face_map[face] = FacePos {
            top_left: pos,
            rotation,
        };

        self.faces_found |= 1u8 << face as u8;

        for direction in 0..4 {
            let offset = OFFSETS[direction];
            let pos = (pos.0 + offset.0 * self.face_length as i32,
                       pos.1 + offset.1 * self.face_length as i32);

            let direction = (direction + rotation) % 4;

            match self.grid.get(pos) {
                Some(b) if b != b' ' => {
                    let link = &FACES[face].links[direction];
                    self.add_face(link.next_face,
                                  pos,
                                  (rotation + link.rotation) % 4);
                },
                _ => (),
            };
        }
    }

    fn act(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Left => self.direction = (self.direction + 3) % 4,
            Action::Right => self.direction = (self.direction + 1) % 4,
            Action::Forward(n) => {
                for _ in 0..n {
                    let (pos, direction) = self.next_pos()?;
                    match self.grid.get(pos).unwrap() {
                        b'.' => {
                            self.pos = pos;
                            self.direction = direction;
                        },
                        _ => break,
                    }
                }
            },
        }

        Ok(())
    }

    fn password(&self) -> i32 {
        (self.pos.1 + 1) * 1000 + (self.pos.0 + 1) * 4 + self.direction as i32
    }

    fn next_pos(&self) -> Result<((i32, i32), usize), String> {
        let offset = OFFSETS[self.direction];
        let next_pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        Ok(match self.grid.get(next_pos) {
            None | Some(b' ') => self.first_pos()?,
            _ => (next_pos, self.direction),
        })
    }

    fn first_pos(&self) -> Result<((i32, i32), usize), String> {
        if self.cube {
            self.cube_first_pos()
        } else {
            Ok(self.simple_first_pos())
        }
    }

    fn simple_first_pos(&self) -> ((i32, i32), usize) {
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
                break (pos, self.direction);
            }

            pos.0 += offset.0;
            pos.1 += offset.1;
        }
    }

    fn cube_first_pos(&self) -> Result<((i32, i32), usize), String> {
        let face_num;

        'found_face: {
            for (i, f) in self.face_map.iter().enumerate() {
                if (f.top_left.0..
                    f.top_left.0 +
                    self.face_length as i32).contains(&self.pos.0) &&
                    (f.top_left.1..
                     f.top_left.1 +
                     self.face_length as i32).contains(&self.pos.1) {
                    face_num = i;
                    break 'found_face;
                }
            }

            return Err(format!("couldn’t find face at {:?}", self.pos));
        }

        let link = &FACES[face_num].links[self.direction];
        let offset = OFFSETS[self.direction];
        let face_x = (self.pos.0 +
                      offset.0 +
                      self.face_length as i32) %
            self.face_length as i32;
        let face_y = (self.pos.1 +
                      offset.1 +
                      self.face_length as i32) %
            self.face_length as i32;
        let rotation = (self.face_map[face_num].rotation + link.rotation) % 4;

        let (face_x, face_y) = match rotation {
            0 => (face_x, face_y),
            1 => (self.face_length as i32 - 1 - face_y, face_x),
            2 => (self.face_length as i32 - 1 - face_x,
                  self.face_length as i32 - 1 - face_y),
            3 => (face_y, self.face_length as i32 - 1 - face_y),
            _ => panic!("impossible rotation"),
        };

        let face_num = link.next_face;

        let res =
            ((self.face_map[face_num].top_left.0 + face_x,
              self.face_map[face_num].top_left.1 + face_y),
             (self.direction + rotation) % 4);

        println!("{:?} {} -> {:?}",
                 self.pos,
                 self.direction,
                 res);
        Ok(res)
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

fn read_input<I: BufRead>(input: &mut I) ->
    Result<(Grid, Box<[Action]>), String>
{
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

    for part in 1..=2 {
        let cube = part == 2;

        let mut state = match State::new(&grid, cube) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(s) => s,
        };

        println!("{:?}", state.face_map);

        for &action in password.iter() {
            if let Err(e) = state.act(action) {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            }
        }

        println!("part {}: {}", part, state.password());
    }

    std::process::ExitCode::SUCCESS
}
