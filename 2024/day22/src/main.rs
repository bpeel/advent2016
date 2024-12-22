use std::process::ExitCode;

fn step(number: u64) -> u64 {
    // Step 1
    let number = ((number << 6) ^ number) & 0xffffff;
    // Step 2
    let number = ((number >> 5) ^ number) & 0xffffff;
    // Step 3
    ((number << 11) ^ number) & 0xffffff
}

fn part1(numbers: &[u64]) {
    let result = numbers.iter().map(|&number| {
        let secret = (0..2000).fold(number, |number, _| step(number));

        secret
    }).sum::<u64>();

    println!("Part 1: {}", result);
}

fn add_to_history(number: u64, history: &mut [u8; 5]) {
    history.copy_within(1.., 0);
    history[4] = (number % 10) as u8;
}

fn history_matches_pattern(history: [u8; 5], pattern: [i8; 4]) -> bool {
    (1..=4).all(|pos| {
        history[pos] as i8 - history[pos - 1] as i8 ==
            pattern[pos - 1]
    })
}

fn find_pattern(mut number: u64, pattern: [i8; 4]) -> Option<u8> {
    let mut history = [0u8; 5];

    for _ in 0..4 {
        add_to_history(number, &mut history);
        number = step(number);
    }

    for _ in 0..2000 - 4 {
        add_to_history(number, &mut history);
        number = step(number);

        if history_matches_pattern(history, pattern) {
            return Some(*history.last().unwrap());
        }
    }

    None
}

fn pattern_is_possible(pattern: [i8; 4]) -> bool {
    for i in 0..3 {
        for j in i + 1..4 {
            if pattern[i..j].iter().sum::<i8>().abs() > 18 {
                return false;
            }
        }
    }

    true
}

fn read_numbers() -> Result<Vec<u64>, String> {
    std::io::stdin().lines().map(|result| {
        let line = result.map_err(|e| e.to_string())?;

        line.parse::<u64>().map_err(|_| format!("invalid number: {}", line))
    }).collect()
}

struct Patterns {
    pattern: [i8; 4],
}

impl Patterns {
    fn new() -> Patterns {
        Patterns {
            pattern: [-10, -9, -9, -9],
        }
    }
}

impl Iterator for Patterns {
    type Item = [i8; 4];

    fn next(&mut self) -> Option<[i8; 4]> {
        for pos in self.pattern.iter_mut() {
            *pos += 1;

            if *pos <= 9 {
                return Some(self.pattern);
            } else {
                *pos = -9;
            }
        }

        None
    }
}

fn score_pattern(numbers: &[u64], pattern: [i8; 4]) -> u64 {
    numbers.iter().map(|&number| {
        find_pattern(number, pattern).unwrap_or(0) as u64
    }).sum::<u64>()
}

fn main() -> ExitCode {
    let numbers = match read_numbers() {
        Ok(numbers) => numbers,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    part1(&numbers);

    let part2 = Patterns::new()
        .filter(|&pattern| pattern_is_possible(pattern))
        .map(|pattern| score_pattern(&numbers, pattern))
        .max()
        .unwrap();

    println!("Part 2: {:?}", part2);

    ExitCode::SUCCESS
}
