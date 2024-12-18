use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub trait Direction: Sized + Clone + Copy {
    type Pos: Clone + Copy;

    fn first_direction() -> Self;
    fn next_direction(self) -> Option<Self>;
    fn move_pos(self, pos: Self::Pos) -> Self::Pos;
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
        QuadDirection::Right
    }

    fn next_direction(self) -> Option<QuadDirection> {
        match self {
            QuadDirection::Right => Some(QuadDirection::Down),
            QuadDirection::Down => Some(QuadDirection::Left),
            QuadDirection::Left => Some(QuadDirection::Up),
            QuadDirection::Up => None,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitResult {
    Continue,
    Backtrack,
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
