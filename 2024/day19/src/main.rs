use std::process::ExitCode;
use regex::Regex;

fn build_regex(towels: &str) -> Regex {
    let mut buf = r"\A(".to_string();

    for (i, towel) in towels.split(", ").enumerate() {
        if i > 0 {
            buf.push('|');
        }

        buf.push_str(&regex::escape(towel));
    }

    buf.push_str(r")+\z$");

    Regex::new(&buf).unwrap()
}

fn main() -> ExitCode {
    let mut count = 0;
    let mut towel_regex: Option<Regex> = None;

    for result in std::io::stdin().lines() {
        let line = match result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
        };

        if let Some(towel_regex) = towel_regex.as_ref() {
            if line.is_empty() {
                continue;
            }

            if towel_regex.is_match(&line) {
                count += 1;
            }
        } else {
            towel_regex = Some(build_regex(&line));
        }
    }

    println!("Part 1: {}", count);

    ExitCode::SUCCESS
}
