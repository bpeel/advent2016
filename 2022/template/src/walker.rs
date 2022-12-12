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
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction for QuadDirection {
    type Pos = (i32, i32);

    fn first_direction() -> QuadDirection {
        QuadDirection::UP
    }

    fn next_direction(self) -> Option<QuadDirection> {
        match self {
            QuadDirection::UP => Some(QuadDirection::DOWN),
            QuadDirection::DOWN => Some(QuadDirection::LEFT),
            QuadDirection::LEFT => Some(QuadDirection::RIGHT),
            QuadDirection::RIGHT => None,
        }
    }

    fn opposite(self) -> QuadDirection {
        match self {
            QuadDirection::UP => QuadDirection::DOWN,
            QuadDirection::DOWN => QuadDirection::UP,
            QuadDirection::LEFT => QuadDirection::RIGHT,
            QuadDirection::RIGHT => QuadDirection::LEFT,
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
            QuadDirection::UP => (0, -1),
            QuadDirection::DOWN => (0, 1),
            QuadDirection::LEFT => (-1, 0),
            QuadDirection::RIGHT => (1, 0),
        }
    }

    pub fn from_char(ch: char) -> Option<QuadDirection> {
        match ch.to_ascii_lowercase() {
            'u' | 'n' => Some(QuadDirection::UP),
            'd' | 's' => Some(QuadDirection::DOWN),
            'l' | 'w' => Some(QuadDirection::LEFT),
            'r' | 'e' => Some(QuadDirection::RIGHT),
            _ => None
        }
    }
}

impl std::str::FromStr for QuadDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_lowercase()[..] {
            "u" | "n" | "up" | "north" => Ok(QuadDirection::UP),
            "d" | "s" | "down" | "south" => Ok(QuadDirection::DOWN),
            "l" | "w" | "left" | "west" => Ok(QuadDirection::LEFT),
            "r" | "e" | "right" | "east" => Ok(QuadDirection::RIGHT),
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
                               QuadDirection::UP);
        check_direction_string(&["down", "d", "DOWN", "D", "Down",
                                 "SoUTH", "s"],
                               QuadDirection::DOWN);
        check_direction_string(&["left", "l", "LEFT", "L", "Left",
                                 "West", "w"],
                               QuadDirection::LEFT);
        check_direction_string(&["right", "r", "RIGHT", "R", "Right",
                                 "East", "E"],
                               QuadDirection::RIGHT);

        for d in "udlr".chars() {
            let d = QuadDirection::from_char(d).unwrap();
            let offset = d.offset();
            let opposite = d.opposite().offset();
            assert_eq!(offset.0, -opposite.0);
            assert_eq!(offset.1, -opposite.1);
        }

        assert_eq!(QuadDirection::UP.offset(), (0, -1));
        assert_eq!(QuadDirection::DOWN.offset(), (0, 1));
        assert_eq!(QuadDirection::LEFT.offset(), (-1, 0));
        assert_eq!(QuadDirection::RIGHT.offset(), (1, 0));
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
