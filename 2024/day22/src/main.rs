use std::process::ExitCode;

fn step(number: u64) -> u64 {
    // Step 1
    let number = ((number << 6) ^ number) % 16777216;
    // Step 2
    let number = ((number >> 5) ^ number) % 16777216;
    // Step 3
    ((number << 11) ^ number) % 16777216
}

fn part1(numbers: &[u64]) {
    let result = numbers.iter().map(|&number| {
        let secret = (0..2000).fold(number, |number, _| step(number));

        println!("{}: {}", number, secret);

        secret
    }).sum::<u64>();

    println!("Part 1: {}", result);
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

    ExitCode::SUCCESS
}
