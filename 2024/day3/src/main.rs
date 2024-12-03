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
    match std::io::read_to_string(std::io::stdin()) {
        Ok(source) => println!("part 1: {}", add_mul(&source)),
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    ExitCode::SUCCESS
}
