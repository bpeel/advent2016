use std::process::ExitCode;

fn add_mul(source: &str) -> u32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(source).map(|caps| {
        let a = caps[1].parse::<u32>().unwrap();
        let b = caps[2].parse::<u32>().unwrap();

        a * b
    }).sum::<u32>()
}

fn enabling_add_mul(mut source: &str) -> u32 {
    let mut sum = 0;

    while let Some((before, after)) = source.split_once("don't()") {
        sum += add_mul(before);

        let Some((_, tail)) = after.split_once("do()")
        else {
            return sum;
        };

        source = tail;
    }

    sum + add_mul(source)
}

fn main() -> ExitCode {
    match std::io::read_to_string(std::io::stdin()) {
        Ok(source) => {
            println!("part 1: {}", add_mul(&source));
            println!("part 2: {}", enabling_add_mul(&source));
        },
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    ExitCode::SUCCESS
}
