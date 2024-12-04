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

    std::process::ExitCode::SUCCESS
}
