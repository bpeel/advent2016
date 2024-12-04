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
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
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
            QuadDirection::Right => Some(QuadDirection::UpRight),
            QuadDirection::UpRight => Some(QuadDirection::DownRight),
            QuadDirection::DownRight => Some(QuadDirection::UpLeft),
            QuadDirection::UpLeft => Some(QuadDirection::DownLeft),
            QuadDirection::DownLeft => None,
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
            QuadDirection::UpRight => (1, -1),
            QuadDirection::DownRight => (1, 1),
            QuadDirection::UpLeft => (-1, -1),
            QuadDirection::DownLeft => (-1, 1),
        }
    }
}

#[derive(Clone)]
pub struct DirectionIter<T: Direction> {
    next: Option<T>,
}

pub fn direction_iter<T: Direction>() -> DirectionIter<T> {
    DirectionIter {
        next: Some(T::first_direction()),
    }
}

impl<T: Direction> Iterator for DirectionIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ret = self.next;

        if let Some(next) = ret {
            self.next = next.next_direction();
        }

        ret
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
