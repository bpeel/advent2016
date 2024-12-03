use std::process::ExitCode;

fn add_mul(source: &str) -> u32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d)\)").unwrap();

    re.captures_iter(source).map(|caps| {
        let a = caps[1].parse::<u32>().unwrap();
        let b = caps[2].parse::<u32>().unwrap();

        a * b
    }).sum::<u32>()
}

fn main() -> ExitCode {
    for line in std::io::stdin().lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
        };

        println!("part 1: {}", add_mul(&line));
    }

   ExitCode::SUCCESS
}
