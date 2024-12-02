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

fn safe(report: &[i32]) -> bool {
    let direction = report[0].cmp(&report[1]);

    if direction == Ordering::Equal {
        return false;
    }

    let mut last = report[0];

    for &level in &report[1..] {
        if last.cmp(&level) != direction {
            return false;
        }

        if last.abs_diff(level) > 3 {
            return false;
        }

        last = level;
    }

    true
}

fn safe2(report: &[i32]) -> bool {
    let mut increases = Vec::new();
    let mut decreases = Vec::new();
    let mut other_bad = None;
    let mut last = report[0];

    for &level in &report[1..] {
        let diff = last.abs_diff(level);

        if diff < 1 || diff > 3 {
            if other_bad.is_some() {
                return false;
            }

            other_bad = Some(level);
        }

        if level > last {
            increases.push(level);
        } else if level < last {
            decreases.push(level);
        }

        last = level;
    }

    if increases.is_empty() || decreases.is_empty() {
        true
    } else if increases.len() > 1 && decreases.len() > 1 {
        false
    } else {
        match other_bad {
            None => true,
            Some(other_bad) => {
                (decreases.len() == 1 && other_bad == decreases[0]) ||
                    (increases.len() == 1 && other_bad == increases[0])
            },
        }
    }
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
        reports.iter().map(|report| safe(report) as u32).sum::<u32>(),
    );

    println!(
        "Part 2: {:?}",
        reports.iter().map(|report| safe2(report) as u32).sum::<u32>(),
    );

    std::process::ExitCode::SUCCESS
}
