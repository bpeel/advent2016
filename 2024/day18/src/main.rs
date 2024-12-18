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

fn find_solution(
    grid_size: (i32, i32),
    bytes: &HashMap<(i32, i32), usize>,
    bytes_dropped: usize,
) -> Option<usize> {
    let mut best = None;

    walker::shortest_walk::<QuadDirection, _>((0, 0), |path, pos| {
        if pos.0 < 0 || pos.0 >= grid_size.0 ||
            pos.1 < 0 || pos.1 >= grid_size.1
        {
            return VisitResult::Backtrack;
        }

        if bytes.get(&pos)
            .map(|&index| index < bytes_dropped)
            .unwrap_or(false)
        {
            return VisitResult::Backtrack;
        }

        if path.len() > best.unwrap_or(usize::MAX) {
            return VisitResult::Backtrack;
        }

        if pos == (grid_size.0 - 1, grid_size.1 - 1) {
            best = Some(path.len());
            VisitResult::Goal
        } else {
            VisitResult::Continue
        }
    });

    best
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

    let best = find_solution(grid_size, &bytes, 1024);

    println!(
        "Part 1: {}",
        best.map(|l| l.to_string()).unwrap_or("no path found".to_string()),
    );

    let mut min = 1;
    let mut max = bytes.len() + 1;

    while max > min {
        let mid = (max + min) / 2;

        if find_solution(grid_size, &bytes, mid).is_some() {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    let first_bad_byte = bytes.iter().find(|(_, &index)| {
        index == min - 1
    }).unwrap().0;

    println!("Part 2: {},{}", first_bad_byte.0, first_bad_byte.1);

    ExitCode::SUCCESS
}
