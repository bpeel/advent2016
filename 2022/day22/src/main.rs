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

const X_FACES: usize = 4;
const Y_FACES: usize = 3;
const N_FACES: usize = X_FACES * Y_FACES;

struct FaceLink {
    next_face: usize,
    direction: usize,
    flip: bool,
}

struct Face {
    links: [Option<FaceLink>; 4],
}

static FACES: [Face; N_FACES] = [
    Face { links: [None, None, None, None] },
    Face { links: [None, None, None, None] },
    Face { links: [
        Some(FaceLink {
            next_face: 11,
            direction: 2,
            flip: true,
        }),
        None,
        Some(FaceLink {
            next_face: 5,
            direction: 1,
            flip: false,
        }),
        Some(FaceLink {
            next_face: 4,
            direction: 1,
            flip: true,
        }),
    ]},
    Face { links: [None, None, None, None] },
    Face { links: [
        None,
        Some(FaceLink {
            next_face: 10,
            direction: 3,
            flip: true,
        }),
        Some(FaceLink {
            next_face: 11,
            direction: 3,
            flip: true,
        }),
        Some(FaceLink {
            next_face: 2,
            direction: 1,
            flip: true,
        }),
    ]},
    Face { links: [
        None,
        Some(FaceLink {
            next_face: 10,
            direction: 0,
            flip: true,
        }),
        None,
        Some(FaceLink {
            next_face: 2,
            direction: 0,
            flip: false,
        }),
    ]},
    Face { links: [
        Some(FaceLink {
            next_face: 11,
            direction: 1,
            flip: true,
        }),
        None,
        None,
        None,
    ]},
    Face { links: [None, None, None, None] },
    Face { links: [None, None, None, None] },
    Face { links: [None, None, None, None] },
    Face { links: [
        None,
        Some(FaceLink {
            next_face: 4,
            direction: 3,
            flip: true,
        }),
        Some(FaceLink {
            next_face: 5,
            direction: 3,
            flip: true,
        }),
        None,
    ]},
    Face { links: [
        Some(FaceLink {
            next_face: 2,
            direction: 2,
            flip: true,
        }),
        Some(FaceLink {
            next_face: 4,
            direction: 0,
            flip: true,
        }),
        None,
        Some(FaceLink {
            next_face: 6,
            direction: 2,
            flip: true,
        }),
    ]},
];

#[derive(Clone, Debug)]
struct State<'a> {
    pos: (i32, i32),
    direction: usize,
    grid: &'a Grid,
    face_length: usize,
    cube: bool,
}

impl<'a> State<'a> {
    fn new(grid: &'a Grid, cube: bool, pos: (i32, i32)) -> State {
        State {
            pos,
            direction: 0,
            grid,
            face_length: grid.width / X_FACES,
            cube,
        }
    }

    fn act(&mut self, action: Action) {
        match action {
            Action::Left => self.direction = (self.direction + 3) % 4,
            Action::Right => self.direction = (self.direction + 1) % 4,
            Action::Forward(n) => {
                for _ in 0..n {
                    let (pos, direction) = self.next_pos();
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
    }

    fn password(&self) -> i32 {
        (self.pos.1 + 1) * 1000 + (self.pos.0 + 1) * 4 + self.direction as i32
    }

    fn next_pos(&self) -> ((i32, i32), usize) {
        let offset = OFFSETS[self.direction];
        let next_pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        match self.grid.get(next_pos) {
            None | Some(b' ') => self.first_pos(),
            _ => (next_pos, self.direction),
        }
    }

    fn first_pos(&self) -> ((i32, i32), usize) {
        if !self.cube {
            return self.simple_first_pos();
        }

        let face_x = self.pos.0 as usize / self.face_length;
        let face_y = self.pos.1 as usize / self.face_length;
        let face_num = face_y * X_FACES + face_x;

        let link = match &FACES[face_num].links[self.direction] {
            None => return self.simple_first_pos(),
            Some(l) => l,
        };

        let face_pos = match self.direction {
            0 | 2 => self.pos.1 % self.face_length as i32,
            1 | 3 => self.pos.0 % self.face_length as i32,
            _ => panic!("impossible direction"),
        };

        let face_pos = if link.flip {
            self.face_length as i32 - 1 - face_pos
        } else {
            face_pos
        };

        let face_x = (link.next_face % X_FACES * self.face_length) as i32;
        let face_y = (link.next_face / X_FACES * self.face_length) as i32;

        let pos = match link.direction {
            0 => (face_x, face_y + face_pos),
            1 => (face_x + face_pos, face_y),
            2 => (face_x + self.face_length as i32 - 1, face_y + face_pos),
            3 => (face_x + face_pos, face_y + self.face_length as i32 - 1),
            _ => panic!("impossible direction"),
        };

        (pos, link.direction)
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

fn validate_links() -> bool {
    for (face_num, face) in FACES.iter().enumerate() {
        for (direction, link) in face.links.iter().enumerate() {
            let link = match link {
                None => continue,
                Some(l) => l,
            };

            let other = match &FACES[link.next_face].links[link.direction ^ 2] {
                None => {
                    eprintln!("{} {} has no return link", face_num, direction);
                    return false;
                },
                Some(o) => o,
            };

            if other.next_face != face_num {
                eprintln!("link from {} {} to {} links back to {}",
                          face_num, direction,
                          link.next_face,
                          other.next_face);
                return false;
            }

            if other.direction != direction ^ 2 {
                eprintln!("link from {} {} doesn’t have right return direction",
                          face_num, direction);
                return false;
            }

            if other.flip != link.flip {
                eprintln!("link from {} {} doesn’t have matching flip",
                          face_num, direction);
                return false;
            }
        }
    }

    true
}

fn main() -> std::process::ExitCode {
    if !(validate_links()) {
        return std::process::ExitCode::FAILURE;
    }

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

    let mut state = State::new(&grid, false, start_pos);

    for &action in password.iter() {
        state.act(action)
    }

    println!("part 1: {}", state.password());

    let mut state = State::new(&grid, true, start_pos);

    for &action in password.iter() {
        state.act(action)
    }

    println!("part 2: {}", state.password());

    std::process::ExitCode::SUCCESS
}
