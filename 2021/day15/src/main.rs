mod util;
mod walker;
use util::Grid;
use walker::{QuadDirection, StackEntry};

const GRID_MULTIPLIER: usize = 5;

fn multiply_grid(old: &Grid) -> Grid {
    let width = old.width * GRID_MULTIPLIER;
    let height = old.height * GRID_MULTIPLIER;

    let mut new = Grid {
        width,
        height,
        values: vec![0; width * height].into_boxed_slice(),
    };

    for y in 0..old.height {
        for x in 0..old.width {
            let old_value = old.values[y * old.width + x] - b'1';
            for outer_y in 0..GRID_MULTIPLIER {
                for outer_x in 0..GRID_MULTIPLIER {
                    let offset = (outer_x + outer_y) as u8;
                    let new_value = (old_value + offset) % 9 + b'1';
                    new.values[(outer_x * old.width + x)
                        + (outer_y * old.height + y) * new.width] = new_value;
                }
            }
        }
    }

    new
}

fn solve(grid: &Grid) -> u64 {
    let mut best_costs = vec![u64::MAX; grid.width * grid.height];
    let mut best_cost = u64::MAX;

    walker::walk((0, 0), |path, pos| {
        if grid.get(pos).is_none() {
            return walker::VisitResult::Backtrack;
        }

        let mut cost = grid.get(pos).unwrap() as u64 - b'0' as u64;
        if path.len() > 1 {
            cost += path[1..].iter().map(|&StackEntry { pos, .. }| grid.get(pos).unwrap() as u64 - b'0' as u64).sum::<u64>();
        }

        if cost >= best_cost {
            return walker::VisitResult::Backtrack;
        }

        let index = pos.1 as usize * grid.width + pos.0 as usize;
        if best_costs[index] <= cost {
            return walker::VisitResult::Backtrack;
        }
        best_costs[index] = cost;

        if pos == (grid.width as i32 - 1, grid.height as i32 - 1) {
            println!("{}", cost);
            best_cost = cost;
            return walker::VisitResult::Goal;
        }

        let mut dirs = [
            QuadDirection::Down,
            QuadDirection::Right,
            QuadDirection::Up,
            QuadDirection::Left,
        ];

        dirs.sort_by_key(|d| match grid.get(d.move_pos(pos)) {
            None => u8::MAX,
            Some(cost) => match d {
                QuadDirection::Up | QuadDirection::Left => cost + 30,
                _ => cost,
            },
        });

        walker::VisitResult::Continue(dirs)
    });

    best_cost
}

fn main() -> std::process::ExitCode {
    let grid = match util::Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    println!("{}", grid);

    let part1 = solve(&grid);

    let grid = multiply_grid(&grid);
    let part2 = solve(&grid);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    std::process::ExitCode::SUCCESS
}
