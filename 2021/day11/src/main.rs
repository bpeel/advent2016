mod util;

use util::Grid;

fn flash(grid: &mut Grid, x: i32, y: i32) {
    for y_off in -1..=1 {
        for x_off in -1..=1 {
            if let Some(value) = grid.get_mut((x + x_off, y + y_off)) {
                if *value <= b'9' {
                    *value += 1;
                }
            }
        }
    }
}

fn step(grid: &mut Grid) -> usize {
    for value in grid.values.iter_mut() {
        *value += 1;
    }

    let mut total_flashes = 0;

    loop {
        let mut flashes = 0;

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.values[y * grid.width + x] == b'9' + 1 {
                    flashes += 1;
                    // Increase the value so we won’t flash it again
                    grid.values[y * grid.width + x] += 1;
                    flash(grid, x as i32, y as i32);
                }
            }
        }

        if flashes <= 0 {
            for value in grid.values.iter_mut() {
                if *value >= b'9' + 1 {
                    *value = b'0';
                }
            }

            break total_flashes;
        }

        total_flashes += flashes;
    }
}

fn main() -> std::process::ExitCode {
    let mut grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let mut total_flashes = 0;

    for step_num in 1.. {
        let flashes = step(&mut grid);

        total_flashes += flashes;

        if step_num == 100 {
            println!("part 1: {}", total_flashes);
        }

        if flashes >= grid.width * grid.height {
            println!("part 2: {}", step_num);
            break;
        }
    }

    std::process::ExitCode::SUCCESS
}
