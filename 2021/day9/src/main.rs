mod util;

use util::Grid;

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

fn main() -> std::process::ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    println!("part 1: {}", part1(&grid));

    std::process::ExitCode::SUCCESS
}
