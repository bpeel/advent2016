use std::cmp::Ordering;

static MARKERS: [&'static str; 2] = [
    "[[2]]",
    "[[6]]",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct List {
    entries: Vec<ListEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListEntry {
    Integer(i32),
    List(List),
}

impl List {
    fn new() -> List {
        List { entries: Vec::<ListEntry>::new() }
    }
}

impl std::fmt::Display for ListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListEntry::Integer(i) => write!(f, "{}", i),
            ListEntry::List(l) => write!(f, "{}", l),
        }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (entry_num, entry) in self.entries.iter().enumerate() {
            if entry_num > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", entry)?;
        }

        write!(f, "]")
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

fn compare_entry(a: &ListEntry, b: &ListEntry) -> Ordering {
    match a {
        ListEntry::Integer(int_a) => {
            match b {
                ListEntry::Integer(int_b) => int_a.cmp(int_b),
                ListEntry::List(list_b) => {
                    let array_a = [ListEntry::Integer(*int_a)];
                    compare_list_slice(&array_a, &list_b.entries)
                },
            }
        },
        ListEntry::List(list_a) => {
            match b {
                ListEntry::Integer(int_b) => {
                    let array_b = [ListEntry::Integer(*int_b)];
                    compare_list_slice(&list_a.entries, &array_b)
                },
                ListEntry::List(list_b) =>
                    compare_list_slice(&list_a.entries, &list_b.entries),
            }
        },
    }
}

fn compare_list_slice(a: &[ListEntry], b: &[ListEntry]) -> Ordering {
    let mut a = a.into_iter();
    let mut b = b.into_iter();

    loop {
        match a.next() {
            Some(entry_a) => {
                match b.next() {
                    None => return Ordering::Greater,
                    Some(entry_b) => match compare_entry(entry_a, entry_b) {
                        Ordering::Equal => (),
                        r @ (Ordering::Greater | Ordering::Less) => return r,
                    }
                }
            },
            None => match b.next() {
                Some(_) => return Ordering::Less,
                None => return Ordering::Equal,
            },
        }
    }
}

fn make_lists_with_dividers(pairs: &[(List, List)]) -> Vec<List> {
    let mut lists: Vec<List> =
        MARKERS.iter().map(|marker| marker.parse::<List>().unwrap()).collect();

    for (a, b) in pairs {
        lists.push(a.clone());
        lists.push(b.clone());
    }

    lists.sort_by(|a, b| compare_list_slice(&a.entries, &b.entries));

    lists
}

fn read_lists<I>(lines: &mut I) -> Result<Vec<(List, List)>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut lists = Vec::<(List, List)>::new();
    let mut list_a = Option::<List>::None;
    let mut need_blank = false;

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        if line.len() == 0 {
            if let Some(_) = list_a {
                return Err(format!("line {}: unpaired list", line_num + 1));
            }
            need_blank = false;
            continue;
        } else if need_blank {
            return Err(format!("line {}: expected blank line", line_num + 1));
        }

        let list = match line.parse::<List>() {
            Ok(list) => list,
            Err(e) => return Err(format!("line {}: {}", line_num + 1, e)),
        };

        match list_a {
            None => list_a = Some(list),
            Some(other_list) => {
                lists.push((other_list, list));
                list_a = None;
                need_blank = true;
            },
        }
    }

    if let Some(_) = list_a {
        return Err("unpaired list at end of file".to_string());
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

    let mut part1 = 0;

    for (pair_num, (a, b)) in lists.iter().enumerate() {
        println!("a: {}\n\
                  b: {}",
                 a, b);
        let comp = compare_list_slice(&a.entries, &b.entries);
        println!("{:?}", comp);

        match comp {
            Ordering::Less | Ordering::Equal => part1 += pair_num + 1,
            Ordering::Greater => (),
        }
    }

    let lists_with_dividers = make_lists_with_dividers(&lists);

    for l in lists_with_dividers.iter() {
        println!("{}", l);
    }

    let part2 = MARKERS.iter().map(|marker| {
        let marker = marker.parse::<List>().unwrap();
        lists_with_dividers.iter().position(|l| l == &marker).unwrap() + 1
    }).fold(1, |a, b| a * b);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    std::process::ExitCode::SUCCESS
}
