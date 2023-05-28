use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

enum SnailFishElement {
    Number(i32),
    Node(Box<SnailFishNumber>),
}

struct SnailFishNumber {
    a: SnailFishElement,
    b: SnailFishElement,
}

struct StackEntry {
    a: Option<SnailFishElement>,
}

impl FromStr for SnailFishNumber {
    type Err = SnailFishError;

    fn from_str(s: &str) -> Result<SnailFishNumber, SnailFishError> {
        let mut iter = s.chars();
        let mut stack = Vec::<StackEntry>::new();

        let num = 'parse_loop: loop {
            match iter.next() {
                None => {
                    return Err(SnailFishError::UnexpectedEnd);
                },
                Some('[') => (),
                Some(_) => {
                    return Err(SnailFishError::InvalidCharacter);
                },
            }

            let a_start = iter.as_str();

            let a = match iter.next() {
                None => {
                    return Err(SnailFishError::UnexpectedEnd);
                },
                Some('[') => {
                    stack.push(StackEntry { a: None });
                    iter = a_start.chars();
                    continue;
                }
                _ => {
                    let Some((number, b_start)) = a_start.split_once(',')
                    else {
                        return Err(SnailFishError::MissingComma);
                    };

                    iter = b_start.chars();

                    SnailFishElement::Number(number.parse::<i32>()?)
                },
            };

            let b_start = iter.as_str();

            let b = match iter.next() {
                None => {
                    return Err(SnailFishError::UnexpectedEnd);
                },
                Some('[') => {
                    stack.push(StackEntry { a: Some(a) });
                    iter = b_start.chars();
                    continue;
                }
                _ => {
                    let Some((number, tail)) = b_start.split_once(']')
                    else {
                        return Err(SnailFishError::UnmatchedBracket);
                    };

                    iter = tail.chars();

                    SnailFishElement::Number(number.parse::<i32>()?)
                },
            };

            let mut num = SnailFishNumber { a, b };

            loop {
                match stack.pop() {
                    Some(StackEntry { a: Some(outer_a) }) => {
                        num = SnailFishNumber {
                            a: outer_a,
                            b: SnailFishElement::Node(Box::new(num)),
                        };
                    },
                    Some(StackEntry { a: None }) => {
                        if !matches!(iter.next(), Some(',')) {
                            return Err(SnailFishError::MissingComma);
                        }

                        stack.push(StackEntry {
                            a: Some(SnailFishElement::Node(Box::new(num)))
                        });

                        continue 'parse_loop;
                    },
                    None => {
                        break 'parse_loop num;
                    }
                }
            }
        };

        if iter.next().is_some() {
            Err(SnailFishError::TrailingData)
        } else {
            Ok(num)
        }
    }
}

impl fmt::Display for SnailFishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.a, self.b)
    }
}

impl fmt::Display for SnailFishElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFishElement::Number(num) => num.fmt(f),
            SnailFishElement::Node(node) => node.fmt(f),
        }
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

#[derive(Debug, Clone)]
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

fn main() {
    println!("Hello, world!");
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
        ];

        for &test in tests.iter() {
            assert_eq!(
                &test.parse::<SnailFishNumber>().unwrap().to_string(),
                test
            );
        }
    }
}
