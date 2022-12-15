#[derive(Debug, Copy, Clone)]
enum ParseResult {
    Incomplete(usize),
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

fn score_missing(missing_chars: &[char]) -> usize {
    let mut score = 0;

    for ch in missing_chars.iter().rev() {
        score = score * 5 + match ch {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("unexpectd missing character"),
        };
    }

    score
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
        ParseResult::Incomplete(score_missing(&stack))
    }
}

fn main() -> std::process::ExitCode {
    let mut part1 = 0;
    let mut part2_scores = Vec::<usize>::new();

    for line in std::io::stdin().lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
        };

        match check_line(&line) {
            ParseResult::Corrupt(score) => part1 += score,
            ParseResult::Incomplete(score) => part2_scores.push(score),
            _ => (),
        }
    }

    println!("part 1: {}", part1);

    part2_scores.sort();

    println!("part 2: {}", part2_scores[part2_scores.len() / 2]);

    std::process::ExitCode::SUCCESS
}
