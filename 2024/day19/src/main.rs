use std::process::ExitCode;

fn count_arrangements(towel_set: &[String], pattern: &str) -> u32 {
    let mut stack = vec![(0, pattern)];
    let mut count = 0;

    while let Some((next_towel, remaining_pattern)) = stack.pop() {
        if next_towel + 1 < towel_set.len() {
            stack.push((next_towel + 1, remaining_pattern));
        }

        let towel = &towel_set[next_towel];

        if remaining_pattern.starts_with(towel) {
            let remaining_pattern = &remaining_pattern[towel.len()..];

            if remaining_pattern.is_empty() {
                count += 1;
            } else {
                stack.push((0, remaining_pattern));
            }
        }
    }

    count
}

fn main() -> ExitCode {
    let mut towel_set: Option<Vec<String>> = None;
    let mut part1 = 0u32;
    let mut part2 = 0u32;

    for result in std::io::stdin().lines() {
        let line = match result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
        };

        if let Some(towel_set) = towel_set.as_ref() {
            if line.is_empty() {
                continue;
            }

            let arrangements = count_arrangements(&towel_set, &line);

            part1 += (arrangements >= 1) as u32;
            part2 += arrangements;
        } else {
            towel_set = Some(
                line.split(", ").map(str::to_string).collect::<Vec<_>>()
            );
        }
    }

    println!(
        "Part 1: {}\n\
         Part 2: {}",
        part1,
        part2,
    );

    ExitCode::SUCCESS
}
