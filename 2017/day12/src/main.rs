use std::process::ExitCode;
use std::collections::HashSet;

fn read_programs() -> Result<Vec<Vec<usize>>, String> {
    let re = regex::Regex::new(r"^(\d+) <-> (\d+(?:, \d+)*)$").unwrap();
    let mut programs = Vec::new();

    for (line_num, result) in std::io::stdin().lines().enumerate() {
        let line = result.map_err(|e| e.to_string())?;

        let captures = re.captures(&line).ok_or_else(|| {
            format!("line {}: invalid syntax", line_num + 1)
        })?;

        let Ok(program_num) = captures[1].parse::<usize>()
        else {
            return Err(format!("line {}: invalid program num", line_num + 1));
        };

        if program_num != programs.len() {
            return Err(format!(
                "line {}: programs out of sequence",
                line_num + 1
            ));
        }

        let mut links = Vec::new();

        for link_str in captures[2].split(", ") {
            let Ok(link) = link_str.parse::<usize>()
            else {
                return Err(format!(
                    "line {}: invalid link number",
                    line_num + 1,
                ));
            };

            links.push(link);
        }

        programs.push(links);
    }

    Ok(programs)
}

fn group_size(programs: &[Vec<usize>], start: usize) -> usize {
    let mut visited = HashSet::new();
    let mut stack = vec![start];

    while let Some(program_num) = stack.pop() {
        if let Some(links) = programs.get(program_num) {
            if visited.contains(&program_num) {
                continue;
            }

            visited.insert(program_num);

            stack.extend_from_slice(&links);
        }
    }

    visited.len()
}

fn main() -> ExitCode {
    let programs = match read_programs() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    println!("Part 1: {}", group_size(&programs, 0));

    ExitCode::SUCCESS
}
