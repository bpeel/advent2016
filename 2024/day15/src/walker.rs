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

    pub fn from_char(ch: char) -> Option<QuadDirection> {
        match ch.to_ascii_lowercase() {
            'u' | 'n' | '^' => Some(QuadDirection::Up),
            'd' | 's' | 'v' => Some(QuadDirection::Down),
            'l' | 'w' | '<' => Some(QuadDirection::Left),
            'r' | 'e' | '>' => Some(QuadDirection::Right),
            _ => None
        }
    }
}
