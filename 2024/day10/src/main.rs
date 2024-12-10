mod util;
mod walker;

use std::process::ExitCode;
use util::Grid;
use std::collections::HashSet;
use walker::VisitResult;
use itertools::Itertools;

fn find_summits(grid: &Grid, trailhead: (i32, i32)) -> u32 {
    let mut n_summits = 0;
    let mut visited_locations = HashSet::new();

    walker::walk::<walker::QuadDirection, _>(trailhead, |path, pos| {
        let Some(height) = grid.get(pos)
        else {
            return VisitResult::Backtrack;
        };

        if visited_locations.contains(&pos) {
            return VisitResult::Backtrack;
        }

        if let Some(last_move) = path.last() {
            let last_height = grid.get(last_move.1).unwrap();

            if height != last_height + 1 {
                return VisitResult::Backtrack;
            }
        }

        visited_locations.insert(pos);

        if height == b'9' {
            n_summits += 1;
            VisitResult::Backtrack
        } else {
            VisitResult::Continue
        }
    });

    n_summits
}

fn part1(grid: &Grid) -> u32 {
    (0..grid.width).cartesian_product(0..grid.height)
        .filter_map(|(x, y)| {
            let pos = (x as i32, y as i32);

            grid.get(pos).and_then(|height| {
                (height == b'0').then(|| find_summits(grid, pos))
            })
        })
        .sum::<u32>()
}

fn main() -> ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    println!("Part 1: {}", part1(&grid));

    ExitCode::SUCCESS
}
