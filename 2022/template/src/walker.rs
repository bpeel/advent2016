use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn next_direction(self) -> Option<Direction> {
        match self {
            Direction::UP => Some(Direction::DOWN),
            Direction::DOWN => Some(Direction::LEFT),
            Direction::LEFT => Some(Direction::RIGHT),
            Direction::RIGHT => None,
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }

    pub fn offset(self) -> (i32, i32) {
        match self {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
        }
    }

    pub fn move_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.offset();
        (x + dx, y + dy)
    }

    pub fn revert_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.offset();
        (x - dx, y - dy)
    }

    pub fn from_char(ch: char) -> Option<Direction> {
        match ch.to_ascii_lowercase() {
            'u' | 'n' => Some(Direction::UP),
            'd' | 's' => Some(Direction::DOWN),
            'l' | 'w' => Some(Direction::LEFT),
            'r' | 'e' => Some(Direction::RIGHT),
            _ => None
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_lowercase()[..] {
            "u" | "n" | "up" | "north" => Ok(Direction::UP),
            "d" | "s" | "down" | "south" => Ok(Direction::DOWN),
            "l" | "w" | "left" | "west" => Ok(Direction::LEFT),
            "r" | "e" | "right" | "east" => Ok(Direction::RIGHT),
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

pub fn walk<F>(start_pos: (i32, i32), mut visit_func: F)
    where F: FnMut(&[Direction], (i32, i32)) -> VisitResult
{
    let mut stack = vec![Direction::RIGHT];
    let mut pos = start_pos;

    loop {
        match visit_func(&stack[1..], pos) {
            VisitResult::STOP => break,
            VisitResult::CONTINUE => {
                stack.push(Direction::UP);
                pos = Direction::UP.move_pos(pos);
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

pub fn shortest_walk<F>(start_pos: (i32, i32), mut visit_func: F)
    where F: FnMut(&[Direction], (i32, i32)) -> VisitResult
{
    let mut shortest_visits = HashMap::<(i32, i32), usize>::new();

    walk(start_pos, |path, pos| {
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
    fn test_direction() {
        fn check_direction_string(tests: &[&str], direction: Direction) {
            for s in tests {
                match s.parse::<Direction>() {
                    Ok(d) => assert_eq!(d, direction),
                    Err(..) => panic!("failed to parse {} into direction", s),
                }

                let first_ch = s.chars().next().unwrap();

                match Direction::from_char(first_ch) {
                    Some(d) => assert_eq!(d, direction),
                    None => panic!("failed to parse {} into direction",
                                   first_ch),
                }
            }
        }
        check_direction_string(&["up", "u", "UP", "U", "Up", "NoRTH", "n"],
                               Direction::UP);
        check_direction_string(&["down", "d", "DOWN", "D", "Down",
                                 "SoUTH", "s"],
                               Direction::DOWN);
        check_direction_string(&["left", "l", "LEFT", "L", "Left",
                                 "West", "w"],
                               Direction::LEFT);
        check_direction_string(&["right", "r", "RIGHT", "R", "Right",
                                 "East", "E"],
                               Direction::RIGHT);

        for d in "udlr".chars() {
            let d = Direction::from_char(d).unwrap();
            let offset = d.offset();
            let opposite = d.opposite().offset();
            assert_eq!(offset.0, -opposite.0);
            assert_eq!(offset.1, -opposite.1);
        }

        assert_eq!(Direction::UP.offset(), (0, -1));
        assert_eq!(Direction::DOWN.offset(), (0, 1));
        assert_eq!(Direction::LEFT.offset(), (-1, 0));
        assert_eq!(Direction::RIGHT.offset(), (1, 0));
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

        shortest_walk((1, 1), |path, (x, y)| {
            if grid.values[y as usize * grid.width + x as usize] != b' ' {
                return VisitResult::BACKTRACK;
            }

            assert!(!visited.contains(&(x, y)));
            visited.insert((x, y));

            if (x, y) == (3, 2) {
                let expected_path: Vec<Direction> = "dddrrrrrruullll"
                    .chars()
                    .map(|c| Direction::from_char(c).unwrap())
                    .collect();
                assert_eq!(path, &expected_path);
            }

            VisitResult::CONTINUE
        });

        assert!(visited.contains(&(3, 2)));
    }
}
