mod util;
mod walker;

use std::process::ExitCode;
use util::Grid;
use walker::{QuadDirection, VisitResult};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn dir_as_clock(dir: QuadDirection) -> u8 {
    match dir {
        QuadDirection::Left => 0,
        QuadDirection::Down => 1,
        QuadDirection::Right => 2,
        QuadDirection::Up => 3,
    }
}

fn turn_difference(a: QuadDirection, b: QuadDirection) -> u8 {
    (dir_as_clock(a) + 4 - dir_as_clock(b)) % 4
}

fn score_path<I>(
    dirs: I,
) -> u32
where I: IntoIterator<Item = QuadDirection>
{
    let mut last_dir = QuadDirection::Right;
    let mut score = 0u32;

    for dir in dirs {
        score += match turn_difference(last_dir, dir) {
            1 | 3 => 1001,
            2 => 2001,
            0 => 1,
            _=> unreachable!("bad turn difference"),
        };

        last_dir = dir;
    }

    score
}

fn main() -> ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let Some(start_pos) = grid.values.iter().position(|&ch| {
        ch == b'S'
    }).map(|index| {
        (
            (index % grid.width) as i32,
            (index / grid.width) as i32,
        )
    })
    else {
        eprintln!("grid has no start");
        return ExitCode::FAILURE;
    };

    let mut visited = HashMap::new();
    let mut best = None;

    walker::walk::<QuadDirection, _>(start_pos, |path, pos| {
        let Some(ch) = grid.get(pos)
        else {
            return VisitResult::Backtrack;
        };

        if ch == b'#' {
            return VisitResult::Backtrack;
        }

        let score = score_path(path.iter().map(|&(dir, _pos)| dir));

        match visited.entry(pos) {
            Entry::Occupied(mut e) => {
                if *e.get() <= score {
                    return VisitResult::Backtrack;
                } else {
                    *e.get_mut() = score;
                }
            },
            Entry::Vacant(e) => {
                e.insert(score);
            },
        }

        if ch == b'E' {
            match best {
                Some(old) => {
                    if old > score {
                        best = Some(score);
                    }
                },
                None => best = Some(score),
            }
            VisitResult::Goal
        } else {
            VisitResult::Continue
        }
    });

    print!("Part 1: ");

    match best {
        Some(score) => println!("{}", score),
        None => println!("no route"),
    }

    ExitCode::SUCCESS
}
