use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub trait Direction: Sized + Clone + Copy {
    type Pos: Clone + Copy;

    fn first_direction() -> Self;
    fn next_direction(self) -> Option<Self>;
    fn opposite(self) -> Self;
    fn move_pos(self, pos: Self::Pos) -> Self::Pos;

    fn revert_pos(self, pos: Self::Pos) -> Self::Pos {
        self.opposite().move_pos(pos)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn opposite(self) -> QuadDirection {
        match self {
            QuadDirection::Up => QuadDirection::Down,
            QuadDirection::Down => QuadDirection::Up,
            QuadDirection::Left => QuadDirection::Right,
            QuadDirection::Right => QuadDirection::Left,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitResult {
    CONTINUE,
    BACKTRACK,
    STOP,
    GOAL,
}

pub fn walk<D, F>(start_pos: D::Pos, mut visit_func: F)
    where F: FnMut(&[D], D::Pos) -> VisitResult,
          D: Direction
{
    let mut stack = Vec::<D>::new();
    let mut pos = start_pos;

    loop {
        match visit_func(&stack, pos) {
            VisitResult::STOP => break,
            VisitResult::CONTINUE => {
                let first_direction = D::first_direction();
                stack.push(D::first_direction());
                pos = first_direction.move_pos(pos);
            },
            VisitResult::GOAL | VisitResult::BACKTRACK => {
                loop {
                    let last_direction = match stack.pop() {
                        Some(d) => d,
                        None => return,
                    };

                    pos = last_direction.revert_pos(pos);

                    if let Some(d) = last_direction.next_direction() {
                        stack.push(d);
                        pos = d.move_pos(pos);
                        break;
                    }
                }
            },
        }
    }
}

pub fn shortest_walk<D, F>(start_pos: D::Pos, mut visit_func: F)
    where F: FnMut(&[D], D::Pos) -> VisitResult,
          D: Direction,
          D::Pos: Hash + Eq,
{
    let mut shortest_visits = HashMap::<D::Pos, usize>::new();

    walk::<D, _>(start_pos, |path: &[D], pos| {
        match shortest_visits.entry(pos) {
            Entry::Occupied(mut e) => {
                let old_length = *e.get();

                if path.len() < old_length {
                    let r = visit_func(path, pos);

                    if r != VisitResult::BACKTRACK {
                        *e.get_mut() = path.len();
                    }

                    r
                } else {
                    VisitResult::BACKTRACK
                }
            },
            Entry::Vacant(e) => {
                let r = visit_func(path, pos);

                if r != VisitResult::BACKTRACK {
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

        for d in "udlr".chars() {
            let d = QuadDirection::from_char(d).unwrap();
            let offset = d.offset();
            let opposite = d.opposite().offset();
            assert_eq!(offset.0, -opposite.0);
            assert_eq!(offset.1, -opposite.1);
        }

        assert_eq!(QuadDirection::Up.offset(), (0, -1));
        assert_eq!(QuadDirection::Down.offset(), (0, 1));
        assert_eq!(QuadDirection::Left.offset(), (-1, 0));
        assert_eq!(QuadDirection::Right.offset(), (1, 0));
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
                return VisitResult::BACKTRACK;
            }

            assert!(!visited.contains(&(x, y)));
            visited.insert((x, y));

            if (x, y) == (3, 2) {
                let expected_path: Vec<QuadDirection> = "dddrrrrrruullll"
                    .chars()
                    .map(|c| QuadDirection::from_char(c).unwrap())
                    .collect();
                assert_eq!(path, &expected_path);
            }

            VisitResult::CONTINUE
        });

        assert!(visited.contains(&(3, 2)));
    }
}
