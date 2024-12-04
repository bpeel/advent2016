mod util;
mod walker;

use walker::QuadDirection;

static WORD: [u8; 4] = *b"XMAS";

fn is_word<D: walker::Direction<Pos = (i32, i32)>>(
    grid: &util::Grid,
    x: usize,
    y: usize,
    direction: D,
) -> bool {
    let mut pos = (x as i32, y as i32);

    for &letter in WORD.iter() {
        if grid.get(pos) != Some(letter) {
            return false;
        }

        pos = direction.move_pos(pos);
    }

    true
}

fn part1(grid: &util::Grid) -> u32 {
    let mut count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            for dir in walker::direction_iter::<QuadDirection>() {
                if is_word(&grid, x, y, dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_ms(a: u8, b: u8) -> bool {
    if a == b'M' {
        b == b'S'
    } else if a == b'S' {
        b == b'M'
    } else {
        false
    }
}

fn part2(grid: &util::Grid) -> u32 {
    let mut count = 0;

    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            if grid.values[y * grid.width + x] != b'A' {
                continue;
            }

            if is_ms(
                grid.values[(y + 1) * grid.width + x + 1],
                grid.values[(y - 1) * grid.width + x - 1],
            ) &&
                is_ms(
                    grid.values[(y + 1) * grid.width + x - 1],
                    grid.values[(y - 1) * grid.width + x + 1],
                )
            {
                count += 1
            }
        }
    }

    count
}

fn main() -> std::process::ExitCode {
    let grid = {
        let mut input = std::io::stdin().lock();

        match util::Grid::load(&mut input) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(grid) => grid,
        }
    };

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    std::process::ExitCode::SUCCESS
}
