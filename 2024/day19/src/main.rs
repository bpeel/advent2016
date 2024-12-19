use std::process::ExitCode;
use std::collections::HashMap;

struct Counter<'s, 'p> {
    towel_set: &'s [String],
    pattern: &'p str,
    cache: HashMap<usize, u32>,
}

impl<'s, 'p> Counter<'s, 'p> {
    fn new(towel_set: &'s [String], pattern: &'p str) -> Counter<'s, 'p> {
        Counter {
            towel_set,
            pattern,
            cache: HashMap::new(),
        }
    }

    fn count_arrangements(&mut self, start: usize) -> u32 {
        if start >= self.pattern.len() {
            1
        } else if let Some(&count) = self.cache.get(&start) {
            count
        } else {
            let count = self.towel_set.iter().filter_map(|towel| {
                self.pattern[start..].starts_with(towel).then(|| {
                    self.count_arrangements(start + towel.len())
                })
            }).sum::<u32>();

            self.cache.insert(start, count);

            count
        }
    }
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

            let arrangements = Counter::new(towel_set, &line)
                .count_arrangements(0);

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
