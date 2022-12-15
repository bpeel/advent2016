#[derive(Debug, Copy, Clone)]
enum ParseResult {
    Incomplete,
    Corrupt(usize),
    Complete,
}

fn opposite_bracket(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unknown bracket"),
    }
}

fn score_character(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unknown character"),
    }
}

fn check_line(line: &str) -> ParseResult {
    let mut stack = Vec::<char>::new();

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(opposite_bracket(ch)),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(expected) if expected == ch => (),
                _ => return ParseResult::Corrupt(score_character(ch)),
            }
            _ => return ParseResult::Corrupt(0),
        };
    }

    if stack.is_empty() {
        ParseResult::Complete
    } else {
        ParseResult::Incomplete
    }
}

fn main() -> std::process::ExitCode {
    let mut part1 = 0;

    for line in std::io::stdin().lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
        };

        if let ParseResult::Corrupt(score) = check_line(&line) {
            part1 += score;
        }
    }

    println!("part 1: {}", part1);

    std::process::ExitCode::SUCCESS
}
