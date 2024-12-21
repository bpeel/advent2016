use std::process::ExitCode;
use std::fmt;

static DIGIT_POSITIONS: [(u8, u8); 10] = [
    (1, 3), // 0

    (0, 2), // 1
    (1, 2), // 2
    (2, 2), // 3

    (0, 1), // 4
    (1, 1), // 5
    (2, 1), // 6

    (7, 0), // 7
    (8, 0), // 8
    (9, 0), // 9
];

const KEYPAD_A: (u8, u8) = (2, 3);
const KEYPAD_BAD: (u8, u8) = (0, 3);

const DPAD_A: (u8, u8) = (2, 0);
const DPAD_BAD: (u8, u8) = (0, 0);

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn as_position(&self) -> (u8, u8) {
        match self {
            Direction::Up => (1, 0),
            Direction::Down => (1, 1),
            Direction::Left => (0, 1),
            Direction::Right => (2, 1),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

struct Movements {
    n_vertical: u8,
    vertical_direction: Direction,
    n_horizontal: u8,
    horizontal_direction: Direction,

    option: u16,
    n_options: u16,

    bad_option: u16,
}

impl Movements {
    fn new(source: (u8, u8), dest: (u8, u8), bad: (u8, u8)) -> Movements {
        let n_horizontal = source.0.abs_diff(dest.0);
        let n_vertical = source.1.abs_diff(dest.1);

        let horizontal_direction = if dest.0 < source.0 {
            Direction::Left
        } else {
            Direction::Right
        };

        let vertical_direction = if dest.1 < source.1 {
            Direction::Up
        } else {
            Direction::Down
        };

        let bad_option = if (source.0 == bad.0 || dest.0 == bad.0) &&
            (source.1 == bad.1 || dest.1 == bad.1)
        {
            // We’re assuming the bad button is on a corner of the
            // grid and that it’s not the destination or the source,
            // so in that case if it’s an option then it’s either on
            // the same column or row as the start button and the bad
            // path would be to move all the way along that axis
            // first.
            if source.0 == bad.0 {
                // Don’t do all the Y movements first
                (1 << n_vertical) - 1
            } else {
                // Don’t do all the X movements first
                ((1 << n_vertical) - 1) << n_horizontal
            }
        } else {
            u16::MAX
        };

        Movements {
            n_vertical,
            n_horizontal,
            horizontal_direction,
            vertical_direction,

            option: 0,
            n_options: 1u16 << (n_vertical + n_horizontal),

            bad_option,
        }
    }
}

impl Iterator for Movements {
    type Item = Path;

    fn next(&mut self) -> Option<Path> {
        while self.option < self.n_options {
            let option = self.option;
            self.option += 1;

            if option != self.bad_option &&
                option.count_ones() == self.n_vertical as u32
            {
                return Some(Path::new(
                    self.n_vertical + self.n_horizontal,
                    self.horizontal_direction,
                    self.vertical_direction,
                    option
                ));
            }
        }

        None
    }
}

#[derive(Clone)]
struct Path {
    n_entries: u8,
    horizontal_direction: Direction,
    vertical_direction: Direction,
    pattern: u16,

    pos: u8,
}

impl Path {
    fn new(
        n_entries: u8,
        horizontal_direction: Direction,
        vertical_direction: Direction,
        pattern: u16,
    ) -> Path {
        Path {
            n_entries,
            horizontal_direction,
            vertical_direction,
            pattern,
            pos: 0,
        }
    }
}

impl Iterator for Path {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        (self.pos < self.n_entries).then(|| {
            let dir = if self.pattern & (1 << self.pos) == 0 {
                self.horizontal_direction
            } else {
                self.vertical_direction
            };

            self.pos += 1;

            dir
        })
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for dir in self.clone() {
            dir.fmt(f)?;
        }

        Ok(())
    }
}

fn main() -> ExitCode {
    for path in Movements::new((1, 2), (5, 4), (1, 3)) {
        println!("{}", path);
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path() {
        assert_eq!(
            &Path::new(
                5,
                Direction::Left,
                Direction::Up,
                0x3,
            ).to_string(),
            "^^<<<",
        );

        assert_eq!(
            &Path::new(
                6,
                Direction::Right,
                Direction::Down,
                0xa,
            ).to_string(),
            ">v>v>>",
        );

        assert!(
            &Path::new(
                0,
                Direction::Right,
                Direction::Down,
                0,
            ).next().is_none()
        );
    }

    #[test]
    fn bad() {
        let mut movements = Movements::new(
            (2, 3), (0, 2), (0, 3)
        ).map(|path| path.to_string());

        assert_eq!(&movements.next().unwrap(), "^<<");
        assert_eq!(&movements.next().unwrap(), "<^<");
        assert!(movements.next().is_none());

        let mut movements = Movements::new(
            (2, 3), (0, 2), (2, 2)
        ).map(|path| path.to_string());

        assert_eq!(&movements.next().unwrap(), "<^<");
        assert_eq!(&movements.next().unwrap(), "<<^");
        assert!(movements.next().is_none());
    }
}
