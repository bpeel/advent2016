use std::process::ExitCode;
use std::collections::HashMap;

struct NumberCounter {
    nums: HashMap<u64, u32>,
}

impl NumberCounter {
    fn new() -> NumberCounter {
        NumberCounter {
            nums: HashMap::new(),
        }
    }

    fn add(&mut self, num: u64) {
        self.nums.entry(num)
            .and_modify(|n| *n += 1 )
            .or_insert(1);
    }

    fn remove(&mut self, num: u64) {
        if let Some(n) = self.nums.get_mut(&num) {
            *n -= 1;

            if *n <= 0 {
                self.nums.remove(&num);
            }
        } else {
            unreachable!();
        }
    }

    fn contains(&self, num: u64) -> bool {
        match self.nums.get(&num) {
            Some(n) => {
                assert!(*n > 0);
                true
            },
            None => false
        }
    }
}

fn read_numbers() -> Result<Vec<u64>, String> {
    let mut numbers = Vec::<u64>::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(e.to_string()),
        };

        numbers.push(match line.parse::<u64>() {
            Ok(num) => num,
            Err(e) => return Err(format!("line {}: {}", line_num + 1, e)),
        });
    }

    Ok(numbers)
}

fn part1(n_preceding_numbers: usize, numbers: &[u64]) -> Option<u64> {
    let mut sums = NumberCounter::new();

    for i in 0..n_preceding_numbers {
        for j in i + 1..n_preceding_numbers {
            sums.add(numbers[i] + numbers[j]);
        }
    }

    for index in n_preceding_numbers..numbers.len() {
        let num = numbers[index];

        if !sums.contains(num) {
            return Some(num);
        }

        let to_remove = numbers[index - n_preceding_numbers];

        for n in &numbers[index - n_preceding_numbers + 1..index] {
            sums.add(num + n);
            sums.remove(to_remove + n);
        }
    }

    None
}

fn main() -> ExitCode {
    let n_preceding_numbers = if let Some(n) = std::env::args().nth(1) {
        match n.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("usage: day9 [n_preceding_numbers]");
                return ExitCode::FAILURE;
            },
        }
    } else {
        25
    };

    let numbers = match read_numbers() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    if numbers.len() < n_preceding_numbers {
        eprintln!(
            "Need at least {} numbers but {} were received",
            n_preceding_numbers,
            numbers.len(),
        );
        return ExitCode::FAILURE;
    }

    match part1(n_preceding_numbers, &numbers) {
        Some(n) => println!("part 1: {}", n),
        None => println!("par 1: All numbers were a sum of two previous nums"),
    }

    ExitCode::SUCCESS
}
