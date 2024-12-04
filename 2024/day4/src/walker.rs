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
