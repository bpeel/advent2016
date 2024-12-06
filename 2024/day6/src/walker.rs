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

    pub fn turn_right(self) -> QuadDirection {
        match self {
            QuadDirection::Up => QuadDirection::Right,
            QuadDirection::Right => QuadDirection::Down,
            QuadDirection::Down => QuadDirection::Left,
            QuadDirection::Left => QuadDirection::Up,
        }
    }
}
