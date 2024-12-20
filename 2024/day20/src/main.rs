mod util;

use std::process::ExitCode;
use util::Grid;
use std::collections::VecDeque;

struct Cheat {
    _start: (u16, u16),
    _end: (u16, u16),
    saving: u32,
}

struct Cheats<'a> {
    route: &'a [(u16, u16)],
    next_start: usize,
    next_end: usize,
    max_distance: u32,
}

impl<'a> Cheats<'a> {
    fn new(route: &'a [(u16, u16)], max_distance: u32) -> Cheats<'a> {
        Cheats {
            route,
            next_start: 0,
            next_end: 1,
            max_distance,
        }
    }
}

impl<'a> Iterator for Cheats<'a> {
    type Item = Cheat;

    fn next(&mut self) -> Option<Cheat> {
        while self.next_start < self.route.len() {
            let start = self.next_start;

            while self.next_end < self.route.len() {
                let end = self.next_end;
                self.next_end += 1;

                let distance =
                    self.route[start].0.abs_diff(self.route[end].0) +
                    self.route[start].1.abs_diff(self.route[end].1);

                if (distance as u32) <= self.max_distance &&
                    (distance as usize) < end - start
                {
                    return Some(Cheat {
                        _start: self.route[start],
                        _end: self.route[end],
                        saving: end as u32 - start as u32 - distance as u32,
                    });
                }
            }

            self.next_start += 1;
            self.next_end = self.next_start + 1;
        }

        None
    }
}

fn build_route(grid: &Grid) -> Result<Vec<(u16, u16)>, String> {
    let start = grid.values.iter().position(|&b| b == b'S')
        .ok_or_else(|| "grid has no start".to_string())?;

    let mut route = Vec::new();
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = vec![false; grid.values.len()];

    // Breadth-first search the route so we can build it up in order
    while let Some((pos, len)) = queue.pop_front() {
        if grid.values[pos] == b'#' || visited[pos] {
            continue;
        }

        visited[pos] = true;

        let x = pos % grid.width;

        route.push((x as u16, (pos / grid.width) as u16));

        if pos >= grid.width {
            queue.push_back((pos - grid.width, len + 1));
        }
        if pos + grid.width < grid.values.len() {
            queue.push_back((pos + grid.width, len + 1));
        }
        if x >= 1 {
            queue.push_back((pos - 1, len + 1));
        }
        if x + 1 < grid.width {
            queue.push_back((pos + 1, len + 1));
        }
    }

    if !route.last().map(|&(x, y)| {
        grid.values[y as usize * grid.width + x as usize] == b'E'
    }).unwrap_or(false)
    {
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

    for (i, max_distance) in [2, 20].into_iter().enumerate() {
        println!(
            "Part {}: {}",
            i + 1,
            Cheats::new(&route, max_distance)
                .filter(|cheat| cheat.saving >= 100)
                .count(),
        );
    }

    ExitCode::SUCCESS
}
