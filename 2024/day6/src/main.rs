use std::process::ExitCode;

mod util;
mod walker;

use util::Grid;
use walker::QuadDirection;

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

fn is_loop(grid: &mut Grid, mut pos: (i32, i32)) -> bool {
    let mut direction = QuadDirection::Up;

    *grid.get_mut(pos).unwrap() = b'0' + (1u8 << direction as u8);

    loop {
        let next_pos = direction.move_pos(pos);

        match grid.get(next_pos) {
            Some(b'#') => direction = direction.turn_right(),
            Some(grid_byte) => {
                pos = next_pos;

                if grid_byte == b'.' {
                    *grid.get_mut(pos).unwrap() =
                        b'0' + (1u8 << direction as u8);
                } else {
                    // Have we already been here in the same direction?
                    let found_loop = grid_byte & (1u8 << direction as u8) != 0;
                    *grid.get_mut(pos).unwrap() |= 1u8 << direction as u8;
                    if found_loop {
                        break true;
                    }
                }
            },
            None => break false,
        }
    }
}

fn part2(grid: &Grid, pos: (i32, i32)) -> usize {
    let mut count = 0;
    let mut grid_copy = grid.clone();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.values[y * grid.width + x] != b'.' {
                continue;
            }

            grid.values.clone_into(&mut grid_copy.values);
            grid_copy.values[y * grid.width + x] = b'#';

            if is_loop(&mut grid_copy, pos) {
                count += 1;
            }
        }
    }

    count
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
    println!("Part 2: {}", part2(&grid, start));

    ExitCode::SUCCESS
}
