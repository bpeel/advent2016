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

fn add_group_nodes(
    programs: &[Vec<usize>],
    start: usize,
    visited: &mut HashSet<usize>,
) {
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
}

fn main() -> ExitCode {
    let programs = match read_programs() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let mut visited = HashSet::new();
    let mut n_groups = 0usize;

    for program_num in 0..programs.len() {
        if visited.contains(&program_num) {
            continue;
        }

        add_group_nodes(&programs, program_num, &mut visited);

        if program_num == 0 {
            println!("Part 1: {}", visited.len());
        }

        n_groups += 1;
    }

    println!("Part 2: {}", n_groups);

    ExitCode::SUCCESS
}
