mod walker;

use std::process::ExitCode;
use walker::{QuadDirection, VisitResult};
use std::collections::HashMap;

fn read_bytes() -> Result<HashMap<(i32, i32), usize>, String> {
    let re = regex::Regex::new(r"^(\d+),(\d+)$").unwrap();
    let mut bytes = Vec::new();

    for (line_num, result) in std::io::stdin().lines().enumerate() {
        let line = result.map_err(|e| e.to_string())?;

        let captures = re.captures(&line).ok_or_else(|| {
            format!("line {}: invalid syntax", line_num + 1)
        })?;

        let mut parts = [0; 2];

        for (i, part) in parts.iter_mut().enumerate() {
            *part = captures[i + 1].parse::<i32>().map_err(|_| {
                format!("line {}: bad number", line_num + 1)
            })?;
        }

        bytes.push((parts[0], parts[1]));
    }

    Ok(HashMap::from_iter(
        bytes.into_iter().enumerate().map(|(index, pos)| (pos, index))
    ))
}

fn get_grid_size() -> Result<(i32, i32), String> {
    let mut parts = [71, 71];

    for (i, arg) in std::env::args().skip(1).take(2).enumerate() {
        parts[i] = arg.parse::<i32>().map_err(|_| {
            format!("bad argument: {}", arg)
        })?;
    }

    Ok((parts[0], parts[1]))
}

fn main() -> ExitCode {
    let bytes = match read_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let grid_size = match get_grid_size() {
        Ok(size) => size,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let mut best = None;

    for y in 0..grid_size.1 {
        for x in 0..grid_size.0 {
            if bytes.get(&(x, y)).map(|&index| index < 1024).unwrap_or(false) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }

    walker::shortest_walk::<QuadDirection, _>((0, 0), |path, pos| {
        if pos.0 < 0 || pos.0 >= grid_size.0 ||
            pos.1 < 0 || pos.1 >= grid_size.1
        {
            return VisitResult::Backtrack;
        }

        if bytes.get(&pos).map(|&index| index < 1024).unwrap_or(false) {
            return VisitResult::Backtrack;
        }

        if pos == (grid_size.0 - 1, grid_size.1 - 1) {
            best = Some(path.len());
            VisitResult::Goal
        } else {
            VisitResult::Continue
        }
    });

    println!(
        "Part 1: {}",
        best.map(|l| l.to_string()).unwrap_or("no path found".to_string()),
    );

    ExitCode::SUCCESS
}
