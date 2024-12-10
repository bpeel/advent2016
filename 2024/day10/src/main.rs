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

fn count_trails(grid: &Grid, trailhead: (i32, i32)) -> u32 {
    let mut n_trails = 0;

    walker::walk::<walker::QuadDirection, _>(trailhead, |path, pos| {
        let Some(height) = grid.get(pos)
        else {
            return VisitResult::Backtrack;
        };

        if let Some(last_move) = path.last() {
            let last_height = grid.get(last_move.1).unwrap();

            if height != last_height + 1 {
                return VisitResult::Backtrack;
            }
        }

        // Don’t allow loops in the trail
        if path.iter().find(|(_, old_pos)| pos == *old_pos).is_some() {
            return VisitResult::Backtrack;
        }

        if height == b'9' {
            n_trails += 1;
            VisitResult::Backtrack
        } else {
            VisitResult::Continue
        }
    });

    n_trails
}

fn score_grid<F>(
    grid: &Grid,
    mut rate_trailhead: F,
) -> u32
    where F: FnMut(&Grid, (i32, i32)) -> u32
{
    (0..grid.width).cartesian_product(0..grid.height)
        .filter_map(|(x, y)| {
            let pos = (x as i32, y as i32);

            grid.get(pos).and_then(|height| {
                (height == b'0').then(|| rate_trailhead(grid, pos))
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

    println!("Part 1: {}", score_grid(&grid, find_summits));
    println!("Part 2: {}", score_grid(&grid, count_trails));

    ExitCode::SUCCESS
}
