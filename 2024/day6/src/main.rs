use std::process::ExitCode;

mod util;
mod walker;

use util::Grid;
use walker::QuadDirection;
use walker::Direction;

fn find_start(grid: &Grid) -> Option<(i32, i32)> {
    grid.values.iter()
        .position(|&value| value == b'^')
        .map(|position| {
            (
                (position % grid.width) as i32,
                (position / grid.height) as i32,
            )
        })
}

fn part1(mut grid: Grid, mut pos: (i32, i32)) -> usize {
    let mut count = 1;
    let mut direction = QuadDirection::Up;

    *grid.get_mut(pos).unwrap() = b'X';

    loop {
        let next_pos = direction.move_pos(pos);

        match grid.get(next_pos) {
            Some(b'#') => direction = direction.turn_right(),
            Some(b'X') => pos = next_pos,
            Some(_) => {
                count += 1;
                pos = next_pos;
                *grid.get_mut(pos).unwrap() = b'X';
            },
            None => break count,
        }
    }
}

fn main() -> ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let Some(start) = find_start(&grid)
    else {
        eprintln!("grid has no start position");
        return ExitCode::FAILURE;
    };

    println!("Part 1: {}", part1(grid.clone(), start));

    ExitCode::SUCCESS
}
