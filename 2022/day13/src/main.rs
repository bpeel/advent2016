#[derive(Debug, Clone)]
struct List {
    entries: Vec<ListEntry>,
}

#[derive(Debug, Clone)]
enum ListEntry {
    Integer(i32),
    List(List),
}

impl List {
    fn new() -> List {
        List { entries: Vec::<ListEntry>::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseState {
    AwaitingFirstItem,
    AwaitingComma,
    AwaitingNextItem,
    InInteger,
}

use ParseState::*;

impl std::str::FromStr for List {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::<List>::new();
        let mut state = AwaitingFirstItem;

        for (ch_pos, ch) in s.char_indices() {
            match ch {
                '[' => match state {
                    AwaitingFirstItem | AwaitingNextItem => {
                        stack.push(List::new());
                        state = AwaitingFirstItem;
                    },
                    AwaitingComma | InInteger => {
                        return Err("unexpected ‘[’".to_string());
                    },
                },
                ']' => {
                    if state == AwaitingNextItem {
                        return Err("expected ‘[’ or digit".to_string());
                    }
                    match stack.pop() {
                        None => return Err("unmatched ‘]’".to_string()),
                        Some(list) => {
                            if stack.len() == 0 {
                                if ch_pos + ch.len_utf8() < s.len() {
                                    return Err("extra data at end of string"
                                               .to_string());
                                }
                                return Ok(list);
                            } else {
                                let entry = ListEntry::List(list);
                                stack.last_mut().unwrap().entries.push(entry);
                            }
                        },
                    }
                    state = AwaitingComma;
                },
                ',' => match state {
                    AwaitingComma | InInteger => {
                        state = AwaitingNextItem;
                    },
                    AwaitingFirstItem | AwaitingNextItem => {
                        return Err("unexpected ‘,’".to_string());
                    },
                },
                ' ' => match state {
                    AwaitingFirstItem |
                    AwaitingComma |
                    AwaitingNextItem => (),
                    InInteger => {
                        state = AwaitingComma;
                    },
                },
                digit @ '0'..='9' => match state {
                    AwaitingFirstItem |
                    AwaitingNextItem => {
                        if stack.len() == 0 {
                            return Err("integer outside of list".to_string());
                        }
                        let digit = digit.to_digit(10).unwrap();
                        let entry = ListEntry::Integer(digit as i32);
                        stack.last_mut().unwrap().entries.push(entry);
                        state = InInteger;
                    },
                    AwaitingComma => {
                        return Err("unexpected digit".to_string());
                    },
                    InInteger => {
                        let entry = &mut stack
                            .last_mut().unwrap()
                            .entries
                            .last_mut().unwrap();
                        if let ListEntry::Integer(ref mut n) = entry {
                            *n = (*n * 10) + digit.to_digit(10).unwrap() as i32;
                        } else {
                            panic!("should be parsing an integer");
                        }
                    }
                }
                _ => return Err(format!("unexpected char ‘{}’", ch)),
            }
        }

        Err("list not terminated".to_string())
    }
}

fn read_lists<I>(lines: &mut I) -> Result<Vec<List>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut lists = Vec::<List>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let list = match line.parse::<List>() {
            Ok(list) => list,
            Err(e) => return Err(format!("line: {}: {}",
                                         line_num + 1,
                                         e)),
        };

        println!("{:?}", list);

        lists.push(list);
    }

    Ok(lists)
}

fn main() -> std::process::ExitCode {
    let lists = match read_lists(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(lists) => lists,
    };

    println!("{:?}", lists);

    std::process::ExitCode::SUCCESS
}
