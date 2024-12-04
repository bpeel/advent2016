use std::process::ExitCode;

static DIGIT_NAMES: [&'static str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn begins_with_digit_or_name(s: &str) -> Option<u32> {
    if let Some(digit) = s.chars().next() {
        if digit.is_ascii_digit() {
            return Some(digit as u32 - '0' as u32);
        }
    }

    for (i, &name) in DIGIT_NAMES.iter().enumerate() {
        if s.starts_with(name) {
            return Some(i as u32 + 1);
        }
    }

    None
}

fn first_digit(s: &str) -> Option<u32> {
    let mut chars = s.chars();

    loop {
        if let Some(digit) = begins_with_digit_or_name(chars.as_str()) {
            break Some(digit);
        }

        if chars.next().is_none() {
            break None;
        }
    }
}

fn last_digit(s: &str) -> Option<u32> {
    let mut chars = s.chars();

    loop {
        if chars.next_back().is_none() {
            break None;
        }

        let tail = &s[chars.as_str().len()..];

        if let Some(digit) = begins_with_digit_or_name(tail) {
            break Some(digit);
        }
    }
}

fn main() -> std::process::ExitCode {
    let mut part1 = 0;
    let mut part2 = 0;

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let Ok(line) = line
        else {
            eprintln!("error reading stdin");
            return ExitCode::FAILURE;
        };

        let Some(first) = line.find(is_digit)
        else {
            eprintln!("line {} contains no digit", line_num + 1);
            return ExitCode::FAILURE;
        };

        let last = line.rfind(is_digit).unwrap();

        let calibration_value =
            (line.as_bytes()[first] as u32 - '0' as u32) * 10 +
            line.as_bytes()[last] as u32 -
            '0' as u32;

        part1 += calibration_value;

        let first = first_digit(&line).unwrap();
        let last = last_digit(&line).unwrap();

        let calibration_value = first * 10 + last;

        part2 += calibration_value;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    ExitCode::SUCCESS
}
