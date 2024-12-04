use std::process::ExitCode;

fn main() -> std::process::ExitCode {
    let re = regex::Regex::new(r"[^0-9]*([0-9]).*([0-9])").unwrap();
    let mut part1 = 0;

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let Ok(line) = line
        else {
            eprintln!("error reading stdin");
            return ExitCode::FAILURE;
        };

        let Some(captures) = re.captures(&line)
        else {
            eprintln!("line {} does not match regex", line_num + 1);
            return ExitCode::FAILURE;
        };

        let calibration_value = (captures[0].chars().next().unwrap() as u32 -
                                 '0' as u32) * 10 +
            (captures[1].chars().next().unwrap() as u32 - '0' as u32);

        part1 += calibration_value;
    }

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
