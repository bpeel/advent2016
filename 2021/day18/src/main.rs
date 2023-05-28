use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

enum SnailFishNumber {
    Integer(i32),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>),
}

struct StackEntry {
    a: Option<SnailFishNumber>,
}

impl FromStr for SnailFishNumber {
    type Err = SnailFishError;

    fn from_str(mut s: &str) -> Result<SnailFishNumber, SnailFishError> {
        let mut stack = Vec::<StackEntry>::new();

        let num = 'parse_loop: loop {
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

            let mut num = SnailFishNumber::Integer(s[0..number_end].parse()?);

            s = &s[number_end..];

            loop {
                match stack.pop() {
                    Some(StackEntry { a: Some(a) }) => {
                        if let Some(tail) = s.strip_prefix(']') {
                            s = tail;
                        } else {
                            return Err(SnailFishError::UnmatchedBracket);
                        }

                        num = SnailFishNumber::Pair(
                            Box::new(a),
                            Box::new(num),
                        );
                    },
                    Some(StackEntry { a: None }) => {
                        if let Some(tail) = s.strip_prefix(',') {
                            s = tail;
                        } else {
                            return Err(SnailFishError::MissingComma);
                        }

                        stack.push(StackEntry {
                            a: Some(num),
                        });

                        continue 'parse_loop;
                    },
                    None => {
                        break 'parse_loop num;
                    }
                }
            }
        };

        if !s.is_empty() {
            Err(SnailFishError::TrailingData)
        } else {
            Ok(num)
        }
    }
}

impl fmt::Display for SnailFishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFishNumber::Integer(num) => num.fmt(f),
            SnailFishNumber::Pair(a, b) => write!(f, "[{},{}]", a, b),
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
            "12",
        ];

        for &test in tests.iter() {
            assert_eq!(
                &test.parse::<SnailFishNumber>().unwrap().to_string(),
                test
            );
        }
    }
}
