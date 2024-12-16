use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub trait Direction: Sized + Clone + Copy {
    type Pos: Clone + Copy;

    fn first_direction() -> Self;
    fn next_direction(self) -> Option<Self>;
    fn move_pos(self, pos: Self::Pos) -> Self::Pos;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuadDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Direction for QuadDirection {
    type Pos = (i32, i32);

    fn first_direction() -> QuadDirection {
        QuadDirection::Up
    }

    fn next_direction(self) -> Option<QuadDirection> {
        match self {
            QuadDirection::Up => Some(QuadDirection::Down),
            QuadDirection::Down => Some(QuadDirection::Left),
            QuadDirection::Left => Some(QuadDirection::Right),
            QuadDirection::Right => None,
        }
    }

    fn move_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.offset();
        (x + dx, y + dy)
    }
}

impl QuadDirection {
    pub fn offset(self) -> (i32, i32) {
        match self {
            QuadDirection::Up => (0, -1),
            QuadDirection::Down => (0, 1),
            QuadDirection::Left => (-1, 0),
            QuadDirection::Right => (1, 0),
        }
    }

    pub fn from_char(ch: char) -> Option<QuadDirection> {
        match ch.to_ascii_lowercase() {
            'u' | 'n' => Some(QuadDirection::Up),
            'd' | 's' => Some(QuadDirection::Down),
            'l' | 'w' => Some(QuadDirection::Left),
            'r' | 'e' => Some(QuadDirection::Right),
            _ => None
        }
    }
}

impl std::str::FromStr for QuadDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_lowercase()[..] {
            "u" | "n" | "up" | "north" => Ok(QuadDirection::Up),
            "d" | "s" | "down" | "south" => Ok(QuadDirection::Down),
            "l" | "w" | "left" | "west" => Ok(QuadDirection::Left),
            "r" | "e" | "right" | "east" => Ok(QuadDirection::Right),
            _ => Err(format!("unknown direction: {}", s)),
        }
    }
}

// HexDirection is meant to be used on a grid of hexagonal spaces.
// Every odd line is shifted to the right by 0.5 spaces. So space 0 on
// line 1 is down and to the right of space 0 on line 0.
//
//   ╱╲╱╲╱╲
//  │0│1│2│ line 0
//   ╲╱╲╱╲╱╲
//   │0│1│2│ line 1
//    ╲╱╲╱╲╱

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HexDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Left,
    Right,
}

impl Direction for HexDirection {
    type Pos = (i32, i32);

    fn first_direction() -> HexDirection {
        HexDirection::UpLeft
    }

    fn next_direction(self) -> Option<HexDirection> {
        match self {
            HexDirection::UpLeft => Some(HexDirection::UpRight),
            HexDirection::UpRight => Some(HexDirection::DownLeft),
            HexDirection::DownLeft => Some(HexDirection::DownRight),
            HexDirection::DownRight => Some(HexDirection::Left),
            HexDirection::Left => Some(HexDirection::Right),
            HexDirection::Right => None,
        }
    }

    fn move_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            HexDirection::UpLeft => (x - (!y & 1), y - 1),
            HexDirection::UpRight => (x + (y & 1), y - 1),
            HexDirection::DownLeft => (x - (!y & 1), y + 1),
            HexDirection::DownRight => (x + (y & 1), y + 1),
            HexDirection::Left => (x - 1, y),
            HexDirection::Right => (x + 1, y),
        }
    }
}

// TriangleDirection is meant to be used on a grid of triangle spaces.
// Every is connected to its left and right neighbours and one
// neighbour on a nother row. Whether the row neighbour is above or
// below depends on whether the row and column are odd or even.
//
//   ╱0╲1╱2╲3╱ line 0
//   ╲0╱1╲2╱3╲ line 1

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TriangleDirection {
    Left,
    Right,
    Row,
}

impl Direction for TriangleDirection {
    type Pos = (i32, i32);

    fn first_direction() -> TriangleDirection {
        TriangleDirection::Left
    }

    fn next_direction(self) -> Option<TriangleDirection> {
        match self {
            TriangleDirection::Left => Some(TriangleDirection::Right),
            TriangleDirection::Right => Some(TriangleDirection::Row),
            TriangleDirection::Row => None,
        }
    }

    fn move_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            TriangleDirection::Left => (x - 1, y),
            TriangleDirection::Right => (x + 1, y),
            TriangleDirection::Row => (x, y + ((x ^ y ^ 1) & 1) * 2 - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitResult {
    Continue,
    Backtrack,
    Stop,
    Goal,
}

pub fn walk<D, F>(start_pos: D::Pos, mut visit_func: F)
    where F: FnMut(&[(D, D::Pos)], D::Pos) -> VisitResult,
          D: Direction
{
    let mut stack = Vec::<(D, D::Pos)>::new();
    let mut pos = start_pos;

    loop {
        match visit_func(&stack, pos) {
            VisitResult::Stop => break,
            VisitResult::Continue => {
                let first_direction = D::first_direction();
                stack.push((D::first_direction(), pos));
                pos = first_direction.move_pos(pos);
            },
            VisitResult::Goal | VisitResult::Backtrack => {
                loop {
                    let (last_direction, last_pos) = match stack.pop() {
                        Some(s) => s,
                        None => return,
                    };

                    pos = last_pos;

                    if let Some(d) = last_direction.next_direction() {
                        stack.push((d, last_pos));
                        pos = d.move_pos(pos);
                        break;
                    }
                }
            },
        }
    }
}

pub fn shortest_walk<D, F>(start_pos: D::Pos, mut visit_func: F)
    where F: FnMut(&[(D, D::Pos)], D::Pos) -> VisitResult,
          D: Direction,
          D::Pos: Hash + Eq,
{
    let mut shortest_visits = HashMap::<D::Pos, usize>::new();

    walk::<D, _>(start_pos, |path: &[(D, D::Pos)], pos| {
        match shortest_visits.entry(pos) {
            Entry::Occupied(mut e) => {
                let old_length = *e.get();

                if path.len() < old_length {
                    let r = visit_func(path, pos);

                    if r != VisitResult::Backtrack {
                        *e.get_mut() = path.len();
                    }

                    r
                } else {
                    VisitResult::Backtrack
                }
            },
            Entry::Vacant(e) => {
                let r = visit_func(path, pos);

                if r != VisitResult::Backtrack {
                    e.insert(path.len());
                }

                r
            },
        }
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quad_direction() {
        fn check_direction_string(tests: &[&str], direction: QuadDirection) {
            for s in tests {
                match s.parse::<QuadDirection>() {
                    Ok(d) => assert_eq!(d, direction),
                    Err(..) => panic!("failed to parse {} into direction", s),
                }

                let first_ch = s.chars().next().unwrap();

                match QuadDirection::from_char(first_ch) {
                    Some(d) => assert_eq!(d, direction),
                    None => panic!("failed to parse {} into direction",
                                   first_ch),
                }
            }
        }
        check_direction_string(&["up", "u", "UP", "U", "Up", "NoRTH", "n"],
                               QuadDirection::Up);
        check_direction_string(&["down", "d", "DOWN", "D", "Down",
                                 "SoUTH", "s"],
                               QuadDirection::Down);
        check_direction_string(&["left", "l", "LEFT", "L", "Left",
                                 "West", "w"],
                               QuadDirection::Left);
        check_direction_string(&["right", "r", "RIGHT", "R", "Right",
                                 "East", "E"],
                               QuadDirection::Right);

        assert_eq!(QuadDirection::Up.offset(), (0, -1));
        assert_eq!(QuadDirection::Down.offset(), (0, 1));
        assert_eq!(QuadDirection::Left.offset(), (-1, 0));
        assert_eq!(QuadDirection::Right.offset(), (1, 0));
    }

    #[test]
    fn test_hex_direction() {
        let directions = [HexDirection::UpLeft,
                          HexDirection::UpRight,
                          HexDirection::DownLeft,
                          HexDirection::DownRight,
                          HexDirection::Left,
                          HexDirection::Right];

        assert_eq!(directions[0], HexDirection::first_direction());

        for (i, d) in directions[0..directions.len() - 1].iter().enumerate() {
            assert_eq!(directions[0].next_direction().unwrap(),
                       directions[1]);
        }
        assert_eq!(directions[directions.len() - 1].next_direction(), None);

        assert_eq!(HexDirection::UpLeft.move_pos((0, 0)), (-1, -1));
        assert_eq!(HexDirection::UpLeft.move_pos((0, 1)), (0, 0));
        assert_eq!(HexDirection::UpRight.move_pos((0, 0)), (0, -1));
        assert_eq!(HexDirection::UpRight.move_pos((0, 1)), (1, 0));
        assert_eq!(HexDirection::DownLeft.move_pos((0, 0)), (-1, 1));
        assert_eq!(HexDirection::DownLeft.move_pos((0, 1)), (0, 2));
        assert_eq!(HexDirection::DownRight.move_pos((0, 0)), (0, 1));
        assert_eq!(HexDirection::DownRight.move_pos((0, 1)), (1, 2));
        assert_eq!(HexDirection::Left.move_pos((0, 0)), (-1, 0));
        assert_eq!(HexDirection::Left.move_pos((0, 1)), (-1, 1));
        assert_eq!(HexDirection::Right.move_pos((0, 0)), (1, 0));
        assert_eq!(HexDirection::Right.move_pos((0, 1)), (1, 1));
    }

    #[test]
    fn test_triangle_direction() {
        let directions = [TriangleDirection::Left,
                          TriangleDirection::Right,
                          TriangleDirection::Row];

        assert_eq!(directions[0], TriangleDirection::first_direction());

        for (i, d) in directions[0..directions.len() - 1].iter().enumerate() {
            assert_eq!(directions[0].next_direction().unwrap(),
                       directions[1]);
        }
        assert_eq!(directions[directions.len() - 1].next_direction(), None);

        assert_eq!(TriangleDirection::Left.move_pos((0, 0)), (-1, 0));
        assert_eq!(TriangleDirection::Left.move_pos((0, 1)), (-1, 1));
        assert_eq!(TriangleDirection::Right.move_pos((0, 0)), (1, 0));
        assert_eq!(TriangleDirection::Right.move_pos((0, 1)), (1, 1));
        assert_eq!(TriangleDirection::Row.move_pos((0, 0)), (0, 1));
        assert_eq!(TriangleDirection::Row.move_pos((0, 1)), (0, 0));
        assert_eq!(TriangleDirection::Row.move_pos((1, 0)), (1, -1));
        assert_eq!(TriangleDirection::Row.move_pos((1, 1)), (1, 2));
    }

    #[test]
    fn test_grid_walk() {
        let mut test_input: &[u8] = b"#########\n\
                                      # #######\n\
                                      # #     #\n\
                                      # ##### #\n\
                                      #       #\n\
                                      #########\n";
        let grid = crate::util::Grid::load(&mut test_input).unwrap();

        let mut visited = std::collections::HashSet::<(i32, i32)>::new();

        shortest_walk::<QuadDirection, _>((1, 1), |path, (x, y)| {
            if grid.values[y as usize * grid.width + x as usize] != b' ' {
                return VisitResult::Backtrack;
            }

            assert!(!visited.contains(&(x, y)));
            visited.insert((x, y));

            if (x, y) == (3, 2) {
                let expected_path: Vec<QuadDirection> = "dddrrrrrruullll"
                    .chars()
                    .map(|c| QuadDirection::from_char(c).unwrap())
                    .collect();
                let actual_path: Vec<QuadDirection> = path
                    .iter()
                    .map(|&(dir, _)| dir)
                    .collect();
                assert_eq!(&actual_path, &expected_path);
            }

            VisitResult::Continue
        });

        assert!(visited.contains(&(3, 2)));
    }

    #[test]
    fn test_hex_grid_walk() {
        let mut test_input: &[u8] = b"...\n\
                                      ##.\n\
                                      .#.\n\
                                      .#.\n\
                                      ...\n";
        let grid = crate::util::Grid::load(&mut test_input).unwrap();

        let mut found_end = false;

        shortest_walk::<HexDirection, _>((0, 0), |path, pos| {
            if pos == (0, 2) {
                let dirs: Vec<HexDirection> = path
                    .iter()
                    .map(|&(dir, _)| dir)
                    .collect();
                assert_eq!(&dirs, &[HexDirection::Right,
                                    HexDirection::Right,
                                    HexDirection::DownRight,
                                    HexDirection::DownLeft,
                                    HexDirection::DownRight,
                                    HexDirection::DownLeft,
                                    HexDirection::Left,
                                    HexDirection::UpLeft,
                                    HexDirection::UpLeft]);
                found_end = true;

                VisitResult::Goal
            } else {
                match grid.get(pos) {
                    Some(b'.') => VisitResult::Continue,
                    _ => VisitResult::Backtrack,
                }
            }
        });

        assert!(found_end);
    }
}
