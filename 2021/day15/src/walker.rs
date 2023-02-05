#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuadDirection {
    Up,
    Down,
    Left,
    Right,
}

impl QuadDirection {
    pub fn move_pos(self, (x, y): (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.offset();
        (x + dx, y + dy)
    }

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

pub struct StackEntry {
    pub pos: (i32, i32),
    pub dirs: [QuadDirection; 4],
    pub next_dir: usize,
}

pub fn walk<F>(start_pos: (i32, i32), mut visit_func: F)
    where F: FnMut(&[StackEntry], (i32, i32)) -> VisitResult
{
    let mut stack = Vec::<StackEntry>::new();
    let mut pos = start_pos;

    loop {
        match visit_func(&stack, pos) {
            VisitResult::Continue => {
                let entry = StackEntry {
                    pos,
                    dirs: [
                        QuadDirection::Right,
                        QuadDirection::Down,
                        QuadDirection::Up,
                        QuadDirection::Left,
                    ],
                    next_dir: 1,
                };

                pos = entry.dirs[0].move_pos(pos);
                stack.push(entry);
            },
            VisitResult::Goal | VisitResult::Backtrack => {
                loop {
                    let mut entry = match stack.pop() {
                        Some(s) => s,
                        None => return,
                    };

                    pos = entry.pos;

                    if entry.next_dir < entry.dirs.len() {
                        let dir = entry.dirs[entry.next_dir];
                        entry.next_dir += 1;
                        pos = dir.move_pos(pos);
                        stack.push(entry);
                        break;
                    }
                }
            },
        }
    }
}

