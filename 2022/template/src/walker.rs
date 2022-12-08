use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Clone, Copy)]
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

    fn move_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::UP => (x, y - 1),
            Direction::DOWN => (x, y + 1),
            Direction::LEFT => (x - 1, y),
            Direction::RIGHT => (x + 1, y),
        }
    }

    fn revert_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::UP => (x, y + 1),
            Direction::DOWN => (x, y - 1),
            Direction::LEFT => (x + 1, y),
            Direction::RIGHT => (x - 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VisitResult {
    CONTINUE,
    BACKTRACK,
    STOP,
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
            VisitResult::BACKTRACK => {
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
                    *e.get_mut() = path.len();
                    visit_func(path, pos)
                } else {
                    VisitResult::BACKTRACK
                }
            },
            Entry::Vacant(e) => {
                e.insert(path.len());
                visit_func(path, pos)
            },
        }
    });
}
