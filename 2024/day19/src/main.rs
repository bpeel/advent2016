use std::process::ExitCode;

fn count_arrangements(towel_set: &[String], pattern: &str) -> u64 {
    // Counts for the number of combinations of towels that can be
    // used to build the pattern starting from each point in the
    // string. We will fill this in in reverse.
    let mut position_counts = vec![0; pattern.len()];
    // The entry represents starting after the end of the pattern.
    // That gives the empty string and we can generate that in exactly
    // one way, ie, by using none of the towels.
    position_counts.push(1);

    for pos in (0..pattern.len()).rev() {
        let count = if let Some((_, tail)) = pattern.split_at_checked(pos) {
            towel_set.iter().filter_map(|towel| {
                // If this position starts with this towel then by
                // using this towel we can reach all of the
                // combinations at the position of the end of the
                // towel again
                tail.starts_with(towel).then(|| {
                    position_counts[pos + towel.len()]
                })
            }).sum::<u64>()
        } else {
            0
        };

        position_counts[pos] = count;
    }

    position_counts[0]
}

fn main() -> ExitCode {
    let mut towel_set: Option<Vec<String>> = None;
    let mut part1 = 0u64;
    let mut part2 = 0u64;

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

            let arrangements = count_arrangements(towel_set, &line);

            part1 += (arrangements >= 1) as u64;
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
