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

fn begins_with_digit_or_name(s: &str) -> Option<u32> {
    s.chars()
        .next()
        .and_then(|ch| ch.to_digit(10))
        .or_else(|| {
            DIGIT_NAMES.iter().enumerate().find_map(|(i, &name)| {
                s.starts_with(name).then(|| i as u32 + 1)
            })
        })
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

        let digit_map = |ch: char| ch.to_digit(10);

        let Some(first) = line.chars().find_map(digit_map)
        else {
            eprintln!("line {} contains no digit", line_num + 1);
            return ExitCode::FAILURE;
        };

        let last = line.chars().rev().find_map(digit_map).unwrap();

        let calibration_value = first * 10 + last;

        part1 += calibration_value;

        let digit_map = |(pos, _)| begins_with_digit_or_name(&line[pos..]);

        let first = line.char_indices().find_map(digit_map).unwrap();
        let last = line.char_indices().rev().find_map(digit_map).unwrap();

        let calibration_value = first * 10 + last;

        part2 += calibration_value;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    ExitCode::SUCCESS
}
