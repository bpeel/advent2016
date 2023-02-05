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
        QuadDirection::Down
    }

    fn next_direction(self) -> Option<QuadDirection> {
        match self {
            QuadDirection::Up => Some(QuadDirection::Left),
            QuadDirection::Down => Some(QuadDirection::Right),
            QuadDirection::Left => None,
            QuadDirection::Right => Some(QuadDirection::Up),
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

