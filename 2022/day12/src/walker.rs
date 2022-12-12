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
}

#[derive(Debug, Clone, Copy)]
pub enum VisitResult {
    CONTINUE,
    BACKTRACK,
    GOAL,
}

pub fn walk<F>(start_pos: (i32, i32), mut visit_func: F)
    where F: FnMut(&[Direction], (i32, i32)) -> VisitResult
{
    let mut stack = vec![Direction::RIGHT];
    let mut pos = start_pos;

    loop {
        match visit_func(&stack[1..], pos) {
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

pub fn shortest_walk<F>(start_pos: (i32, i32), mut visit_func: F) ->
    HashMap::<(i32, i32), usize>
    where F: FnMut(&[Direction], (i32, i32)) -> VisitResult
{
    let mut shortest_visits = HashMap::<(i32, i32), usize>::new();

    walk(start_pos, |path, pos| {
        match shortest_visits.entry(pos) {
            Entry::Occupied(mut e) => {
                let old_length = *e.get();

                if path.len() < old_length {
                    let r = visit_func(path, pos);
                    if !matches!(r, VisitResult::BACKTRACK) {
                        *e.get_mut() = path.len();
                    }
                    r
                } else {
                    VisitResult::BACKTRACK
                }
            },
            Entry::Vacant(e) => {
                let r = visit_func(path, pos);

                if !matches!(r, VisitResult::BACKTRACK) {
                    e.insert(path.len());
                }
                r
            },
        }
    });

    shortest_visits
}
