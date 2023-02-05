mod util;
mod walker;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() -> std::process::ExitCode {
    let grid = match util::Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    println!("{}", grid);

    let mut best_costs = HashMap::<(i32, i32), u64>::new();
    let mut best_cost = u64::MAX;

    walker::walk::<walker::QuadDirection, _>((0, 0), |path, pos| {
        if grid.get(pos).is_none() {
            return walker::VisitResult::Backtrack;
        }

        let mut cost = grid.get(pos).unwrap() as u64 - b'0' as u64;
        if path.len() > 1 {
            cost += path[1..].iter().map(|&(_, pos)| grid.get(pos).unwrap() as u64 - b'0' as u64).sum::<u64>();
        }

        if cost >= best_cost {
            return walker::VisitResult::Backtrack;
        }

        match best_costs.entry(pos) {
            Entry::Occupied(mut e) => {
                if *e.get() <= cost {
                    return walker::VisitResult::Backtrack;
                }
                e.insert(cost);
            },
            Entry::Vacant(e) => {
                e.insert(cost);
            },
        }

        if pos == (grid.width as i32 - 1, grid.height as i32 - 1) {
            println!("{}", cost);
            best_cost = cost;
            return walker::VisitResult::Goal;
        }

        walker::VisitResult::Continue
    });

    std::process::ExitCode::SUCCESS
}
