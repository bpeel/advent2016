use std::process::ExitCode;

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn main() -> std::process::ExitCode {
    let mut part1 = 0;

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let Ok(line) = line
        else {
            eprintln!("error reading stdin");
            return ExitCode::FAILURE;
        };

        let Some(first) = line.find(is_digit)
        else {
            eprintln!("line {} contains no digit", line_num + 1);
            return ExitCode::FAILURE;
        };

        let last = line.rfind(is_digit).unwrap();

        let calibration_value =
            (line.as_bytes()[first] as u32 - '0' as u32) * 10 +
            line.as_bytes()[last] as u32 -
            '0' as u32;

        part1 += calibration_value;
    }

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
