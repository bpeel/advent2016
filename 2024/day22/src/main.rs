use std::process::ExitCode;
use std::collections::HashMap;

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

fn patterns_for_number(mut number: u64) -> HashMap<[i8; 4], u8> {
    let mut history = [0u8; 5];

    for _ in 0..4 {
        add_to_history(number, &mut history);
        number = step(number);
    }

    let mut patterns = HashMap::new();

    for _ in 0..2000 - 4 {
        add_to_history(number, &mut history);
        number = step(number);

        let mut pattern = [0i8; 4];

        for (i, diff) in pattern.iter_mut().enumerate() {
            *diff = history[i + 1] as i8 - history[i] as i8;
        }

        patterns.entry(pattern).or_insert(*history.last().unwrap());
    }

    patterns
}

fn part2(numbers: &[u64]) {
    let mut pattern_sales = HashMap::new();

    for &number in numbers.iter() {
        for (pattern, sales) in patterns_for_number(number) {
            *pattern_sales.entry(pattern).or_insert(0u32) += sales as u32;
        }
    }

    println!("Part 2: {}", pattern_sales.values().cloned().max().unwrap_or(0));
}

fn read_numbers() -> Result<Vec<u64>, String> {
    std::io::stdin().lines().map(|result| {
        let line = result.map_err(|e| e.to_string())?;

        line.parse::<u64>().map_err(|_| format!("invalid number: {}", line))
    }).collect()
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
    part2(&numbers);

    ExitCode::SUCCESS
}
