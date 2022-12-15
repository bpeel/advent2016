mod util;
mod walker;

use util::Grid;
use walker::VisitResult;
use std::collections::HashSet;

fn is_low_point(grid: &Grid, pos: (i32, i32)) -> bool {
    let this_height = grid.get(pos).unwrap();

    for y_off in -1..=1 {
        for x_off in -1..=1 {
            if y_off == 0 && x_off == 0 {
                continue;
            }

            if let Some(other_height) = grid.get((pos.0 + x_off,
                                                  pos.1 + y_off)) {
                if other_height <= this_height {
                    return false;
                }
            }
        }
    }

    true
}

fn part1(grid: &Grid) -> usize {
    let mut sum = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = (x as i32, y as i32);

            let this_value = grid.get(pos).unwrap();

            if this_value < b'0' {
                continue;
            }

            if !is_low_point(grid, pos) {
                continue;
            }

            sum += (this_value - b'0') as usize + 1;
        }
    }

    sum
}

fn part2(grid: &Grid) -> String {
    let mut basin_sizes = Vec::<usize>::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = (x as i32, y as i32);

            if !is_low_point(grid, pos) {
                continue;
            }

            let mut visited_locations = HashSet::<(i32, i32)>::new();

            walker::walk::<walker::QuadDirection, _>(pos, |path, pos| {
                let can_visit = match grid.get(pos) {
                    None => false,
                    Some(this_height) if this_height >= b'9' => false,
                    Some(this_height) => match path.last() {
                        Some(&(_, last_pos)) =>
                            this_height == grid.get(last_pos).unwrap() + 1,
                        None => true,
                    },
                };

                if can_visit && !visited_locations.contains(&pos) {
                    visited_locations.insert(pos);
                    VisitResult::Continue
                } else {
                    VisitResult::Backtrack
                }
            });

            basin_sizes.push(visited_locations.len());
        }
    }

    if basin_sizes.len() < 3 {
        return format!("only {} basins found", basin_sizes.len());
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));

    format!("{}", basin_sizes.iter().take(3).fold(1, |a, &b| a * b))
}

fn main() -> std::process::ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    println!("part 1: {}", part1(&grid));
    println!("part 2: {}", part2(&grid));

    std::process::ExitCode::SUCCESS
}
