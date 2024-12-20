mod util;

use std::process::ExitCode;
use util::Grid;

struct Cheat {
    _pos: usize,
    saving: u32,
}

struct Cheats<'a, 'b> {
    grid: &'a Grid,
    route: &'b [Option<u32>],
    next_pos: usize,
}

impl<'a, 'b> Cheats<'a, 'b> {
    fn new(grid: &'a Grid, route: &'b [Option<u32>]) -> Cheats<'a, 'b> {
        Cheats {
            grid,
            route,
            next_pos: 0,
        }
    }

    fn score_pos(&self, pos: usize) -> Option<u32> {
        let mut min = u32::MAX;
        let mut max = 0;

        let mut add_value = |value: u32| {
            min = value.min(min);
            max = value.max(max);
        };

        let x = pos % self.grid.width;

        // Check each vertical and horizontal neighbour to find the
        // minimum and maximum lengths on the route that this position
        // touches

        if pos >= self.grid.width {
            if let Some(len) = self.route[pos - self.grid.width] {
                add_value(len);
            }
        }
        if pos + self.grid.width < self.grid.values.len() {
            if let Some(len) = self.route[pos + self.grid.width] {
                add_value(len);
            }
        }
        if x >= 1 {
            if let Some(len) = self.route[pos - 1] {
                add_value(len);
            }
        }
        if x + 1 < self.grid.width {
            if let Some(len) = self.route[pos + 1] {
                add_value(len);
            }
        }

        (min != u32::MAX && max - min > 2).then(|| max - min - 2)
    }
}

impl<'a, 'b> Iterator for Cheats<'a, 'b> {
    type Item = Cheat;

    fn next(&mut self) -> Option<Cheat> {
        while self.next_pos < self.grid.values.len() {
            let pos = self.next_pos;
            self.next_pos += 1;

            if self.grid.values[pos] != b'#' {
                continue;
            }

            if let Some(saving) = self.score_pos(pos) {
                return Some(Cheat {
                    _pos: pos,
                    saving,
                });
            }
        }

        None
    }
}

fn build_route(grid: &Grid) -> Result<Vec<Option<u32>>, String> {
    let start = grid.values.iter().position(|&b| b == b'S')
        .ok_or_else(|| "grid has no start".to_string())?;
    let end = grid.values.iter().position(|&b| b == b'E')
        .ok_or_else(|| "grid has no end".to_string())?;

    let mut route = vec![None; grid.values.len()];
    let mut stack = vec![(start, 0)];

    while let Some((pos, len)) = stack.pop() {
        if grid.values[pos] == b'#' || route[pos].is_some() {
            continue;
        }

        route[pos] = Some(len);

        let x = pos % grid.width;

        if pos >= grid.width {
            stack.push((pos - grid.width, len + 1));
        }
        if pos + grid.width < grid.values.len() {
            stack.push((pos + grid.width, len + 1));
        }
        if x >= 1 {
            stack.push((pos - 1, len + 1));
        }
        if x + 1 < grid.width {
            stack.push((pos + 1, len + 1));
        }
    }

    if route[end].is_none() {
        return Err("couldn’t find a path to the end".to_string());
    }

    Ok(route)
}

fn main() -> ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let route = match build_route(&grid) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(route) => route,
    };

    println!(
        "Part 1: {}",
        Cheats::new(&grid, &route)
            .filter(|cheat| cheat.saving >= 100)
            .count(),
    );

    ExitCode::SUCCESS
}
