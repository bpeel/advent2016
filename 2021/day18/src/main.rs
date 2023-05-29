use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug, Clone)]
struct SnailFishNumber {
    items: Vec<SnailFishItem>,
    root: usize,
    magazine: Option<usize>,
}

#[derive(Debug, Clone)]
enum SnailFishItem {
    Integer(i32),
    Pair(usize, usize),
    Deleted(Option<usize>),
}

const EXPLODE_DEPTH: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DescendDirection {
    Start,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AddDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct ActionStackEntry {
    pos: usize,
    direction: DescendDirection,
}

#[derive(Debug, Clone, Copy)]
struct MagnitudeStackEntry {
    pos: usize,
    a: Option<i32>,
}

impl SnailFishNumber {
    fn add_item(&mut self, item: SnailFishItem) -> usize {
        match self.magazine {
            Some(deleted) => {
                let SnailFishItem::Deleted(next) = self.items[deleted]
                else { unreachable!(); };

                self.items[deleted] = item;

                self.magazine = next;

                deleted
            },
            None => {
                self.items.push(item);
                self.items.len() - 1
            },
        }
    }

    fn add_to_child(
        &mut self,
        mut child: usize,
        direction: AddDirection,
        amount: i32,
    ) {
        loop {
            match self.items[child] {
                SnailFishItem::Integer(ref mut value) => {
                    *value += amount;
                    break;
                },
                SnailFishItem::Pair(a, b) => {
                    child = match direction {
                        AddDirection::Left => b,
                        AddDirection::Right => a,
                    };
                },
                SnailFishItem::Deleted(..) => unreachable!(),
            }
        }
    }

    fn add_to_neighbour(
        &mut self,
        stack: &[ActionStackEntry; EXPLODE_DEPTH],
        direction: AddDirection,
        amount: i32,
    ) {
        for entry in stack.iter().rev() {
            let SnailFishItem::Pair(a, b) = self.items[entry.pos]
            else { unreachable!(); };

            match direction {
                AddDirection::Left => {
                    if entry.direction == DescendDirection::Right {
                        self.add_to_child(a, AddDirection::Left, amount);
                        break;
                    }
                },
                AddDirection::Right => {
                    if entry.direction == DescendDirection::Left {
                        self.add_to_child(b, AddDirection::Right, amount);
                        break;
                    }
                },
            }
        }
    }

    fn delete_item(&mut self, item: usize) {
        self.items[item] = SnailFishItem::Deleted(self.magazine);
        self.magazine = Some(item);
    }

    fn explode_item(
        &mut self,
        stack: &[ActionStackEntry; EXPLODE_DEPTH],
        child: usize,
    ) {
        let SnailFishItem::Pair(a, b) = self.items[child]
        else { unreachable!() };

        let SnailFishItem::Integer(a_value) = self.items[a]
        else { unreachable!() };

        self.delete_item(a);

        let SnailFishItem::Integer(b_value) = self.items[b]
        else { unreachable!() };

        self.delete_item(b);

        self.items[child] = SnailFishItem::Integer(0);

        self.add_to_neighbour(&stack, AddDirection::Left, a_value);
        self.add_to_neighbour(&stack, AddDirection::Right, b_value);
    }

    fn try_explode(&mut self) -> bool {
        if !matches!(self.items[self.root], SnailFishItem::Pair(..)) {
            return false;
        }

        let mut stack = [ActionStackEntry {
            pos: self.root,
            direction: DescendDirection::Start,
        }; EXPLODE_DEPTH];

        let mut depth = 1;

        while depth > 0 {
            let entry = &mut stack[depth - 1];
            let item = &self.items[entry.pos];
            depth -= 1;

            let &SnailFishItem::Pair(a, b) = item
            else { unreachable!(); };

            let child = match entry.direction {
                DescendDirection::Start => {
                    entry.direction = DescendDirection::Left;
                    a
                },
                DescendDirection::Left => {
                    entry.direction = DescendDirection::Right;
                    b
                },
                DescendDirection::Right => {
                    continue;
                },
            };

            depth += 1;

            if !matches!(self.items[child], SnailFishItem::Pair(..)) {
                continue;
            }

            if depth >= EXPLODE_DEPTH {
                self.explode_item(&stack, child);
                return true;
            }

            stack[depth] = ActionStackEntry {
                pos: child,
                direction: DescendDirection::Start,
            };

            depth += 1;
        }

        false
    }

    fn split_item(&mut self, item: usize) {
        let SnailFishItem::Integer(value) = self.items[item]
        else { unreachable!(); };

        let a_value = value / 2;
        let b_value = (value + 1) / 2;

        let a = self.add_item(SnailFishItem::Integer(a_value));
        let b = self.add_item(SnailFishItem::Integer(b_value));

        self.items[item] = SnailFishItem::Pair(a, b);
    }

    fn try_split(&mut self) -> bool {
        let mut stack = [ActionStackEntry {
            pos: self.root,
            direction: DescendDirection::Start,
        }; EXPLODE_DEPTH + 1];

        let mut depth = 1;

        while depth > 0 {
            let entry = &mut stack[depth - 1];
            depth -= 1;

            match self.items[entry.pos] {
                SnailFishItem::Pair(a, b) => {
                    let child = match entry.direction {
                        DescendDirection::Start => {
                            entry.direction = DescendDirection::Left;
                            a
                        },
                        DescendDirection::Left => {
                            entry.direction = DescendDirection::Right;
                            b
                        },
                        DescendDirection::Right => {
                            continue;
                        },
                    };

                    depth += 1;

                    stack[depth] = ActionStackEntry {
                        pos: child,
                        direction: DescendDirection::Start,
                    };

                    depth += 1;
                },

                SnailFishItem::Integer(value) => {
                    if value >= 10 {
                        self.split_item(entry.pos);
                        return true;
                    }
                },

                SnailFishItem::Deleted(_) => unreachable!(),
            }
        }

        false
    }

    fn magnitude(&self) -> i32 {
        let mut stack = vec![MagnitudeStackEntry {
            pos: self.root,
            a: None,
        }];

        'outer_loop: while let Some(entry) = stack.pop() {
            match self.items[entry.pos] {
                SnailFishItem::Pair(a, b) => {
                    let child = match entry.a {
                        None => a,
                        Some(_) => b,
                    };

                    stack.push(entry);
                    stack.push(MagnitudeStackEntry { pos: child, a: None });
                },

                SnailFishItem::Integer(mut value) => {
                    while let Some(entry) = stack.pop() {
                        match entry.a {
                            Some(a) => value = a * 3 + value * 2,
                            None => {
                                stack.push(MagnitudeStackEntry {
                                    pos: entry.pos,
                                    a: Some(value),
                                });
                                continue 'outer_loop;
                            },
                        }
                    }

                    return value;
                },

                SnailFishItem::Deleted(_) => unreachable!(),
            }
        }

        unreachable!();
    }

    fn add(&mut self, other: &SnailFishNumber) {
        let offset = self.items.len() + 1;

        self.items.push(SnailFishItem::Pair(self.root, other.root + offset));
        self.root = self.items.len() - 1;

        self.items.extend(other.items.iter().enumerate().map(|(pos, item)| {
            match item {
                SnailFishItem::Deleted(_) => {
                    let old_next = self.magazine;
                    self.magazine = Some(pos + offset);
                    SnailFishItem::Deleted(old_next)
                },
                &SnailFishItem::Integer(value) => SnailFishItem::Integer(value),
                SnailFishItem::Pair(a, b) => SnailFishItem::Pair(
                    a + offset,
                    b + offset,
                ),
            }
        }));
    }

    fn reduce(&mut self) {
        while self.try_explode() || self.try_split() {
        }
    }
}

struct StackEntry {
    a: Option<usize>,
}

impl FromStr for SnailFishNumber {
    type Err = SnailFishError;

    fn from_str(mut s: &str) -> Result<SnailFishNumber, SnailFishError> {
        let mut stack = Vec::<StackEntry>::new();
        let mut items = Vec::<SnailFishItem>::new();

        'parse_loop: loop {
            let number_end = s.find(|c: char| !c.is_numeric())
                .unwrap_or_else(|| s.len());

            if number_end == 0 {
                match s.chars().next() {
                    None => {
                        return Err(SnailFishError::UnexpectedEnd);
                    },
                    Some('[') => (),
                    _ => {
                        return Err(SnailFishError::InvalidCharacter);
                    },
                }

                s = &s[1..];

                stack.push(StackEntry { a: None });

                continue;
            }

            items.push(SnailFishItem::Integer(s[0..number_end].parse()?));

            s = &s[number_end..];

            loop {
                match stack.pop() {
                    Some(StackEntry { a: Some(a) }) => {
                        if let Some(tail) = s.strip_prefix(']') {
                            s = tail;
                        } else {
                            return Err(SnailFishError::UnmatchedBracket);
                        }

                        items.push(SnailFishItem::Pair(a, items.len() - 1));
                    },
                    Some(StackEntry { a: None }) => {
                        if let Some(tail) = s.strip_prefix(',') {
                            s = tail;
                        } else {
                            return Err(SnailFishError::MissingComma);
                        }

                        stack.push(StackEntry {
                            a: Some(items.len() - 1),
                        });

                        continue 'parse_loop;
                    },
                    None => {
                        break 'parse_loop;
                    }
                }
            }
        }

        if !s.is_empty() {
            Err(SnailFishError::TrailingData)
        } else {
            let root = items.len() - 1;

            Ok(SnailFishNumber { items, root, magazine: None })
        }
    }
}

enum DisplayEntryPos {
    A,
    B,
    BRACKET,
}

struct DisplayEntry {
    num: usize,
    pos: DisplayEntryPos,
}

impl fmt::Display for SnailFishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stack = vec![DisplayEntry {
            num: self.root,
            pos: DisplayEntryPos::A,
        }];

        while let Some(entry) = stack.pop() {
            let item = &self.items[entry.num];

            match &item {
                SnailFishItem::Integer(value) => {
                    value.fmt(f)?;
                },
                SnailFishItem::Pair(a, b) => {
                    match entry.pos {
                        DisplayEntryPos::A => {
                            write!(f, "[")?;
                            stack.push(DisplayEntry {
                                num: entry.num,
                                pos: DisplayEntryPos::B,
                            });
                            stack.push(DisplayEntry {
                                num: *a,
                                pos: DisplayEntryPos::A,
                            });
                        },
                        DisplayEntryPos::B => {
                            write!(f, ",")?;
                            stack.push(DisplayEntry {
                                num: entry.num,
                                pos: DisplayEntryPos::BRACKET,
                            });
                            stack.push(DisplayEntry {
                                num: *b,
                                pos: DisplayEntryPos::A,
                            });
                        },
                        DisplayEntryPos::BRACKET => {
                            write!(f, "]")?;
                        },
                    }
                },
                SnailFishItem::Deleted(..) => unreachable!(),
            }
        }

        Ok(())
    }
}

impl fmt::Display for SnailFishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFishError::InvalidCharacter => write!(f, "Invalid character"),
            SnailFishError::UnmatchedBracket => write!(f, "Unmatched bracket"),
            SnailFishError::UnexpectedEnd => write!(f, "Unexpected end"),
            SnailFishError::MissingComma => write!(f, "Missing comma"),
            SnailFishError::TrailingData => write!(f, "Trailing data"),
            SnailFishError::InvalidInteger(e) => e.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SnailFishError {
    InvalidCharacter,
    UnmatchedBracket,
    UnexpectedEnd,
    MissingComma,
    TrailingData,
    InvalidInteger(ParseIntError),
}

impl From<ParseIntError> for SnailFishError {
    fn from(e: ParseIntError) -> SnailFishError {
        SnailFishError::InvalidInteger(e)
    }
}

fn part1(numbers: &[SnailFishNumber]) -> i32 {
    let mut value = numbers[0].clone();

    for other in numbers[1..].iter() {
        value.add(other);
        value.reduce();
    }

    value.magnitude()
}

fn part2(numbers: &[SnailFishNumber]) -> i32 {
    let mut best = i32::MIN;

    for a in 0..numbers.len() {
        for b in 0..numbers.len() {
            if a == b {
                continue;
            }

            let mut value = numbers[a].clone();
            value.add(&numbers[b]);
            value.reduce();

            let magnitude = value.magnitude();

            if best < magnitude {
                best = magnitude;
            }
        }
    }

    best
}

fn main() -> std::process::ExitCode {
    let mut numbers = Vec::<SnailFishNumber>::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
        };

        let number = match line.parse::<SnailFishNumber>() {
            Ok(number) => number,
            Err(e) => {
                eprintln!("line {}: {}", line_num + 1, e);
                return std::process::ExitCode::FAILURE;
            }
        };

        numbers.push(number);
    }

    if numbers.is_empty() {
        eprintln!("Empty input");
        return std::process::ExitCode::FAILURE;
    }

    println!("part 1: {}", part1(&numbers));
    println!("part 2: {}", part2(&numbers));

    std::process::ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let tests = [
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
            "12",
        ];

        for &test in tests.iter() {
            assert_eq!(
                &test.parse::<SnailFishNumber>().unwrap().to_string(),
                test
            );
        }
    }

    #[test]
    fn error() {
        assert_eq!(
            "[".parse::<SnailFishNumber>().unwrap_err(),
            SnailFishError::UnexpectedEnd,
        );

        assert_eq!(
            "[a".parse::<SnailFishNumber>().unwrap_err(),
            SnailFishError::InvalidCharacter,
        );

        let SnailFishError::InvalidInteger(int_error) =
            "[999999999999999,9]".parse::<SnailFishNumber>().unwrap_err()
        else { unreachable!() };
        assert_eq!(*int_error.kind(), std::num::IntErrorKind::PosOverflow);

        assert_eq!(
            "[9,1,3]".parse::<SnailFishNumber>().unwrap_err(),
            SnailFishError::UnmatchedBracket,
        );

        assert_eq!(
            "[9]".parse::<SnailFishNumber>().unwrap_err(),
            SnailFishError::MissingComma,
        );

        assert_eq!(
            "[9,1]yes".parse::<SnailFishNumber>().unwrap_err(),
            SnailFishError::TrailingData,
        );
    }

    #[test]
    fn explode() {
        let tests = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
            ),
        ];

        for (number, exploded) in tests.iter() {
            let mut number = number.parse::<SnailFishNumber>().unwrap();
            assert!(number.try_explode());
            assert_eq!(&number.to_string(), exploded);

            // The magazine should contain exactly two items, ie, the
            // integer items deleted from the pair
            let Some(deleted) = number.magazine
            else { unreachable!(); };

            let SnailFishItem::Deleted(Some(deleted)) = number.items[deleted]
            else { unreachable!(); };

            assert!(matches!(
                number.items[deleted],
                SnailFishItem::Deleted(None),
            ));
        }

        assert!(!"[1,2]".parse::<SnailFishNumber>().unwrap().try_explode());
        assert!(!"12".parse::<SnailFishNumber>().unwrap().try_explode());
    }

    #[test]
    fn split() {
        let tests = [
            ("[11,5]", "[[5,6],5]"),
            ("[[[[10,5],1],1],1]", "[[[[[5,5],5],1],1],1]"),
            ("[1,[[[10,5],1],1]]", "[1,[[[[5,5],5],1],1]]"),
        ];

        for (number, split) in tests.iter() {
            let mut number = number.parse::<SnailFishNumber>().unwrap();
            assert!(number.try_split());
            assert_eq!(&number.to_string(), split);
        }

        assert!(!"[1,2]".parse::<SnailFishNumber>().unwrap().try_split());
    }

    #[test]
    fn magnitude() {
        let tests = [
            ("12", 12),
            ("[9,1]", 29),
            ("[1,9]", 21),
            ("[[9,1],[1,9]]", 129),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ];

        for &(number, magnitude) in tests.iter() {
            let number = number.parse::<SnailFishNumber>().unwrap();
            assert_eq!(number.magnitude(), magnitude);
        }
    }

    #[test]
    fn add() {
        let tests = [
            ("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]"),
            ("5", "6", "[5,6]"),
        ];

        for &(a, b, result) in tests.iter() {
            let mut a = a.parse::<SnailFishNumber>().unwrap();
            let b = b.parse::<SnailFishNumber>().unwrap();

            a.add(&b);

            assert_eq!(&a.to_string(), result);
        }
    }
}
