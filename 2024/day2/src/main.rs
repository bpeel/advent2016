use std::io::BufRead;
use std::cmp::Ordering;

fn read_reports<I>(lines: &mut I) -> Result<Vec<Vec<i32>>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut reports = Vec::new();

    for (line_num, result) in lines.enumerate() {
        let mut levels = Vec::new();

        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        for item in line.split_whitespace() {
            let Ok(item) = item.parse::<i32>()
            else {
                return Err(format!("line: {}: invalid syntax",
                                   line_num + 1));
            };

            levels.push(item);
        }

        reports.push(levels);
    }

    Ok(reports)
}

fn safe<'a, I: IntoIterator<Item = &'a i32>>(report: I) -> bool {
    let mut levels = report.into_iter();
    let mut a = *levels.next().unwrap();
    let mut b = *levels.next().unwrap();
    let direction = a.cmp(&b);

    if direction == Ordering::Equal {
        return false;
    }

    loop {
        if a.abs_diff(b) > 3 {
            return false;
        }

        a = b;

        b = match levels.next() {
            Some(&level) => level,
            None => break true,
        };

        if a.cmp(&b) != direction {
            break false;
        }
    }
}

fn safe2(report: &[i32]) -> bool {
    if safe(report) {
        return true;
    }

    for skip in 0..report.len() {
        if safe(report.iter().enumerate().filter_map(|(i, level)| {
            if i == skip {
                None
            } else {
                Some(level)
            }
        })) {
            return true;
        }
    }

    false
}

fn main() -> std::process::ExitCode {
    let reports;

    {
        let input = std::io::stdin().lock();

        reports = match read_reports(&mut input.lines()) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(reports) => reports,
        };
    }

    println!(
        "Part 1: {:?}",
        reports.iter().map(|report| safe(report.iter()) as u32).sum::<u32>(),
    );

    println!(
        "Part 2: {:?}",
        reports.iter().map(|report| safe2(report) as u32).sum::<u32>(),
    );

    std::process::ExitCode::SUCCESS
}
