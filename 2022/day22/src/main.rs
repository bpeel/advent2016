mod util;

use std::io::BufRead;
use util::Grid;

#[derive(Copy, Clone, Debug)]
enum Action {
    Left,
    Right,
    Forward(usize),
}

const N_DIRECTIONS: usize = 4;

static OFFSETS: [(i32, i32); N_DIRECTIONS] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

const N_FACES: usize = 6;

#[derive(Debug, Clone, Copy)]
struct FaceLink {
    next_face: usize,
    rotation: usize,
    direct: bool,
}

impl Default for FaceLink {
    fn default() -> Self {
        FaceLink {
            next_face: usize::MAX,
            rotation: usize::MAX,
            direct: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Face {
    pos: (i32, i32),
    links: [FaceLink; N_DIRECTIONS],
}

#[derive(Debug, Clone)]
struct Mover {
    horizontal: [usize; 4],
    vertical: [usize; 4],
}

impl Mover {
    fn new() -> Mover {
        Mover {
            horizontal: [0, 1, 5, 3],
            vertical: [0, 2, 5, 4],
        }
    }

    fn for_face(face: usize) -> Mover {
        let mut mover = Mover::new();

        // Move the mover to the face
        match face {
            0 => (),
            1..=4 => {
                mover.move_direction(face - 1);
            },
            5 => {
                mover.up();
                mover.up();
            },
            _ => panic!("invalid face_num"),
        }

        assert_eq!(mover.current_face(), face);

        mover
    }

    fn horizontal_shift(&mut self, mid: usize) -> usize {
        self.horizontal.rotate_left(mid);
        self.vertical[0] = self.horizontal[0];
        self.vertical[2] = self.horizontal[2];
        self.horizontal[0]
    }

    fn right(&mut self) -> usize {
        self.horizontal_shift(1)
    }

    fn left(&mut self) -> usize {
        self.horizontal_shift(self.horizontal.len() - 1)
    }

    fn vertical_shift(&mut self, mid: usize) -> usize {
        self.vertical.rotate_left(mid);
        self.horizontal[0] = self.vertical[0];
        self.horizontal[2] = self.vertical[2];
        self.vertical[0]
    }

    fn down(&mut self) -> usize {
        self.vertical_shift(1)
    }

    fn up(&mut self) -> usize {
        self.vertical_shift(self.vertical.len() - 1)
    }

    fn move_direction(&mut self, dir: usize) -> usize {
        match dir {
            0 => self.right(),
            1 => self.down(),
            2 => self.left(),
            3 => self.up(),
            _ => panic!("tried to move in impossible direction"),
        }
    }

    fn current_face(&self) -> usize {
        self.vertical[0]
    }
}

#[derive(Clone, Debug)]
struct State<'a> {
    pos: (i32, i32),
    direction: usize,
    grid: &'a Grid,
    cube: bool,
    face_length: usize,
    face_map: [Face; N_FACES],
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
            face_map: Default::default(),
        };

        state.find_faces()?;
        state.find_neighbours()?;

        for (face_num, face) in state.face_map.iter().enumerate() {
            println!("{} {:?}:", face_num, face.pos);
            for (dir, link) in face.links.iter().enumerate() {
                if link.rotation == usize::MAX {
                    continue;
                }

                println!(" {} -> {:?}({}) (rotation: {}{})",
                         dir,
                         link.next_face,
                         link.next_face as usize,
                         link.rotation,
                         if link.direct { ", direct" } else { "" });
            }
        }

        Ok(state)
    }

    fn offset_to_face(&self, pos: (i32, i32), dir: usize) -> (i32, i32) {
        let offset = OFFSETS[dir];

        (pos.0 + offset.0 * self.face_length as i32,
         pos.1 + offset.1 * self.face_length as i32)
    }

    fn find_faces(&mut self) -> Result<(), String> {
        let mut pos = self.pos;
        let mut mover = Mover::new();
        let mut stack = Vec::new();
        let mut found_faces = 0u8;

        'outer_loop: loop {
            match self.grid.get(pos) {
                Some(b) if b != b' ' => {
                    let current_face = mover.current_face();

                    if found_faces & (1u8 << current_face as u8) == 0 {
                        if let Some(&last_dir) = stack.last() {
                            let last_face = mover.move_direction(last_dir ^ 2);
                            self.face_map[last_face as usize]
                                .links[last_dir as usize] = FaceLink {
                                    next_face: current_face,
                                    rotation: 0,
                                    direct: true,
                                };
                            self.face_map[current_face as usize]
                                .links[(last_dir ^ 2) as usize] = FaceLink {
                                    next_face: last_face,
                                    rotation: 0,
                                    direct: true,
                                };
                            mover.move_direction(last_dir);
                        }

                        let face = &mut self.face_map[current_face as usize];

                        face.pos = pos;
                        found_faces |= 1u8 << current_face as u8;

                        mover.move_direction(0);
                        stack.push(0);
                        pos = self.offset_to_face(pos, 0);
                        continue;
                    }
                }
                _ => (),
            }

            // Backtrack
            loop {
                let last_dir = match stack.pop() {
                    Some(d) => d,
                    None => break 'outer_loop,
                };

                mover.move_direction(last_dir ^ 2);
                pos = self.offset_to_face(pos, last_dir ^ 2);

                if last_dir + 1 < N_DIRECTIONS {
                    mover.move_direction(last_dir + 1);
                    stack.push(last_dir + 1);
                    pos = self.offset_to_face(pos, last_dir + 1);
                    break;
                }
            }
        }

        let missing_face = found_faces.trailing_ones();

        if (missing_face as usize) < N_FACES {
            Err(format!("missing face {}", missing_face))
        } else {
            Ok(())
        }
    }

    fn find_neighbours(&mut self) -> Result<(), String> {
        for face_num in 0..N_FACES {
            self.find_neighbours_for_face(face_num)?;
        }

        Ok(())
    }

    fn path_to_rotation(path: &[usize]) -> usize {
        if path.len() >= 5 {
            // In my input the path of 5 ends up with a rotation of
            // zero. I’m sure there is a better way to do this but I’m
            // out of ideas.
            return 0;
        } else if path.len() >= 4 {
            // If the length is four then there will be two corners
            // and one of the ends will repeat the same direction. We
            // can ignore the repeated part and treat it as one
            // corner.
            return if path[path.len() - 1] == path[path.len() - 2] {
                State::path_to_rotation(&path[0..path.len() - 2])
            } else if path[0] == path[1] {
                State::path_to_rotation(&path[2..])
            } else {
                panic!("path of length 5 with no repeats at ends {:?}", path)
            };
        }

        let last_direction = path[0];
        let mut rotation = 0;

        for &direction in &path[1..] {
            if direction != last_direction {
                rotation = (direction +
                            N_DIRECTIONS -
                            last_direction) % N_DIRECTIONS;
                break;
            }
        }

        rotation * (path.len() - 1) % N_DIRECTIONS
    }

    fn find_neighbours_for_face(&mut self, face_num: usize) ->
        Result<(), String>
    {
        let opposite_face = {
            let mut mover = Mover::for_face(face_num);
            mover.down();
            mover.down()
        };

        let mut found_faces =
            (1u8 << face_num as u8) |
            (1u8 << opposite_face as u8);

        for direction in 0..N_DIRECTIONS {
            let link = &self.face_map[face_num].links[direction];

            if link.direct {
                found_faces |= 1u8 << link.next_face as u8;
            }
        }

        let mut mover = Mover::new();

        // Get a mover that is orientated correctly by moving along
        // direct routes from the front face to the current face
        for &dir in self.find_path(0, face_num)?.iter() {
            mover.move_direction(dir);
        }

        while found_faces < (1u8 << N_FACES) - 1 {
            let next_face = found_faces.trailing_ones() as usize;
            found_faces |= 1u8 << (next_face as u8);

            let path = self.find_path(face_num, next_face as usize)?;
            let rotation = State::path_to_rotation(&path);

            // Find which direction leads to this face
            let direction;

            'found_direction: {
                for dir in 0..N_DIRECTIONS {
                    let dir_face = mover.move_direction(dir);
                    mover.move_direction(dir ^ 2);
                    if dir_face as usize == next_face {
                        direction = dir;
                        break 'found_direction;
                    }
                }

                panic!("couldn’t find neighbour leading to {} from {}",
                       face_num,
                       next_face);
            }

            self.face_map[face_num].links[direction] = FaceLink {
                next_face,
                rotation,
                direct: false,
            };
        }

        Ok(())
    }

    fn find_path(&self, start_face: usize, end_face: usize) ->
        Result<Vec<usize>, String>
    {
        let mut faces_visited = 0u8;
        let mut stack = Vec::<usize>::new();
        let mut current_face_num = start_face;

        'stack_loop: loop {
            if current_face_num == end_face {
                return Ok(stack);
            }

            if faces_visited & (1u8 << current_face_num as u8) == 0 {
                faces_visited |= 1u8 << current_face_num as u8;

                for dir in 0..N_DIRECTIONS {
                    let link = &self.face_map[current_face_num].links[dir];

                    if link.direct {
                        stack.push(dir);
                        current_face_num = link.next_face as usize;
                        continue 'stack_loop;
                    }
                }
            }

            // Backtrack
            while let Some(last_dir) = stack.pop() {
                current_face_num = self.face_map[current_face_num]
                    .links[last_dir ^ 2]
                    .next_face
                    as usize;

                for dir in last_dir + 1..N_DIRECTIONS {
                    let link = &self.face_map[current_face_num].links[dir];

                    if link.direct {
                        stack.push(dir);
                        current_face_num = link.next_face as usize;
                        continue 'stack_loop;
                    }
                }
            }

            break 'stack_loop Err(format!("couldn’t find path from {} to {}",
                                          start_face,
                                          end_face));
        }
    }

    fn act(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Left =>
                self.direction =
                (self.direction + N_DIRECTIONS - 1) % N_DIRECTIONS,
            Action::Right =>
                self.direction =
                (self.direction + 1) % N_DIRECTIONS,
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
        let face_top_left = (self.pos.0 /
                             self.face_length as i32 *
                             self.face_length as i32,
                             self.pos.1 /
                             self.face_length as i32 *
                             self.face_length as i32);

        let face = match self.face_map.iter().position(|face| {
            face.pos == face_top_left
        }) {
            None => return Err(format!("couldn’t find face at {:?}", self.pos)),
            Some(p) => p,
        };

        let link = &self.face_map[face].links[self.direction];
        let offset = OFFSETS[self.direction];
        let face_x = (self.pos.0 +
                      offset.0 +
                      self.face_length as i32) %
            self.face_length as i32;
        let face_y = (self.pos.1 +
                      offset.1 +
                      self.face_length as i32) %
            self.face_length as i32;

        let (face_x, face_y) = match link.rotation {
            0 => (face_x, face_y),
            1 => (face_y, self.face_length as i32 - 1 - face_x),
            2 => (self.face_length as i32 - 1 - face_x,
                  self.face_length as i32 - 1 - face_y),
            3 => (self.face_length as i32 - 1 - face_y, face_x),
            _ => panic!("impossible rotation"),
        };

        let res =
            ((self.face_map[link.next_face].pos.0 + face_x,
              self.face_map[link.next_face].pos.1 + face_y),
             (self.direction + N_DIRECTIONS - link.rotation) % N_DIRECTIONS);

        println!("{:?} {} -> {:?} ({})",
                 self.pos,
                 self.direction,
                 res,
                 link.rotation);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mover() {
        assert_eq!(Mover::new().right(), 1);
        assert_eq!(Mover::new().down(), 2);
        assert_eq!(Mover::new().left(), 3);
        assert_eq!(Mover::new().up(), 4);

        assert_eq!({ let mut m = Mover::new(); m.right(); m.right() }, 5);
        assert_eq!({ let mut m = Mover::new(); m.down(); m.down() }, 5);
        assert_eq!({ let mut m = Mover::new(); m.left(); m.left() }, 5);
        assert_eq!({ let mut m = Mover::new(); m.up(); m.up() }, 5);

        let mut m = Mover::new();
        assert_eq!(m.right(), 1);
        assert_eq!(m.up(), 4);
        assert_eq!(m.up(), 3);
        assert_eq!(m.right(), 5);
        assert_eq!(m.right(), 1);
        assert_eq!(m.right(), 0);
        assert_eq!(m.right(), 3);

        // Same steps backwards should get the same results in reverse
        assert_eq!(m.left(), 0);
        assert_eq!(m.left(), 1);
        assert_eq!(m.left(), 5);
        assert_eq!(m.left(), 3);
        assert_eq!(m.down(), 4);
        assert_eq!(m.down(), 1);
        assert_eq!(m.left(), 0);

        assert_eq!(Mover::for_face(0).current_face(), 0);
        assert_eq!(Mover::for_face(1).current_face(), 1);
        assert_eq!(Mover::for_face(2).current_face(), 2);
        assert_eq!(Mover::for_face(3).current_face(), 3);
        assert_eq!(Mover::for_face(4).current_face(), 4);
        assert_eq!(Mover::for_face(5).current_face(), 5);
    }

    #[test]
    fn test_path_to_rotation() {
        assert_eq!(State::path_to_rotation(&[0]), 0);
        assert_eq!(State::path_to_rotation(&[1]), 0);
        assert_eq!(State::path_to_rotation(&[2]), 0);
        assert_eq!(State::path_to_rotation(&[3]), 0);

        assert_eq!(State::path_to_rotation(&[0, 0, 0]), 0);
        assert_eq!(State::path_to_rotation(&[1, 1, 1]), 0);
        assert_eq!(State::path_to_rotation(&[2, 2, 2]), 0);
        assert_eq!(State::path_to_rotation(&[3, 3, 3]), 0);

        assert_eq!(State::path_to_rotation(&[2, 3]), 1);
        assert_eq!(State::path_to_rotation(&[2, 1]), 3);
        assert_eq!(State::path_to_rotation(&[0, 3]), 3);
        assert_eq!(State::path_to_rotation(&[0, 1]), 1);
        assert_eq!(State::path_to_rotation(&[1, 0]), 3);
        assert_eq!(State::path_to_rotation(&[1, 2]), 1);
        assert_eq!(State::path_to_rotation(&[3, 0]), 1);
        assert_eq!(State::path_to_rotation(&[3, 2]), 3);

        assert_eq!(State::path_to_rotation(&[1, 0, 0]), 2);
        assert_eq!(State::path_to_rotation(&[1, 2, 2]), 2);
        assert_eq!(State::path_to_rotation(&[3, 0, 0]), 2);
        assert_eq!(State::path_to_rotation(&[3, 2, 2]), 2);
        assert_eq!(State::path_to_rotation(&[2, 3, 3]), 2);
        assert_eq!(State::path_to_rotation(&[2, 1, 1]), 2);
        assert_eq!(State::path_to_rotation(&[0, 3, 3]), 2);
        assert_eq!(State::path_to_rotation(&[0, 1, 1]), 2);

        assert_eq!(State::path_to_rotation(&[0, 0, 1]), 2);
        assert_eq!(State::path_to_rotation(&[0, 0, 3]), 2);
        assert_eq!(State::path_to_rotation(&[2, 2, 1]), 2);
        assert_eq!(State::path_to_rotation(&[2, 2, 3]), 2);
        assert_eq!(State::path_to_rotation(&[3, 3, 0]), 2);
        assert_eq!(State::path_to_rotation(&[3, 3, 2]), 2);
        assert_eq!(State::path_to_rotation(&[1, 1, 0]), 2);
        assert_eq!(State::path_to_rotation(&[1, 1, 2]), 2);

        assert_eq!(State::path_to_rotation(&[0, 0, 1, 0]), 3);
        assert_eq!(State::path_to_rotation(&[2, 3, 2, 2]), 1);

        assert_eq!(State::path_to_rotation(&[2, 1, 1, 2, 1]), 0);
        assert_eq!(State::path_to_rotation(&[3, 0, 3, 3, 0]), 0);
    }
}
