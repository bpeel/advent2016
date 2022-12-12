mod util;
mod walker;

fn find_char(grid: &util::Grid, c: u8) -> Option<(i32, i32)> {
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.values[y * grid.width + x] == c {
                return Some((x as i32, y as i32));
            }
        }
    }

    None
}

fn letter_height(c: u8) -> i32 {
    match c {
        b'S' => 0,
        b'E' => (b'z' - b'a') as i32,
        b'a'..=b'z' => (c - b'a') as i32,
        _ => panic!("unknown height {}", c),
    }
}

fn main() -> std::process::ExitCode {
    let grid = match util::Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    if let Some(c) = grid.values.iter()
        .find(|&&c| c != b'S' &&
              c != b'E' &&
              (c < b'a' || c > b'z')) {
        eprintln!("grid contains invalid character: {}", c);
        return std::process::ExitCode::FAILURE;
    }

    let start = match find_char(&grid, b'S') {
        Some(n) => n,
        None => {
            eprintln!("grid has no start");
            return std::process::ExitCode::FAILURE;
        },
    };
    let end = match find_char(&grid, b'E') {
        Some(n) => n,
        None => {
            eprintln!("grid has no end");
            return std::process::ExitCode::FAILURE;
        },
    };

    // Do the search in reverse, ie, we start from the end point and
    // return a map of shortest distances from all the points that can
    // be reached walking from the end point.
    let distances = walker::shortest_walk(end, |path, (xp, yp)| {
        if xp < 0 || yp < 0 {
            return walker::VisitResult::BACKTRACK;
        }

        let x = xp as usize;
        let y = yp as usize;

        if x >= grid.width || y >= grid.height {
            return walker::VisitResult::BACKTRACK;
        }

        if path.len() > 0 {
            let (last_x, last_y) = path[path.len() - 1].revert_pos((xp, yp));
            let last_height = grid.values[last_x as usize +
                                          last_y as usize * grid.width];
            let last_height = letter_height(last_height);
            let this_height = grid.values[x + y * grid.width];
            let this_height = letter_height(this_height);

            if last_height - this_height > 1 {
                return walker::VisitResult::BACKTRACK;
            }
        }

        if grid.values[x + y * grid.width] == b'S' {
            return walker::VisitResult::GOAL;
        }

        return walker::VisitResult::CONTINUE;
    });

    let part1 = match distances.get(&start) {
        Some(length) => length.to_string(),
        None => "no path found".to_string(),
    };
    println!("part 1: {}", part1);

    // Pick the shortest path out of all the points with zero height
    let part2 = match distances
        .iter()
        .filter_map(|(&(x, y), &v)| {
            let l = grid.values[x as usize + y as usize * grid.width];
            if letter_height(l) == 0 {
                Some(v)
            } else {
                None
            }
        }).min() {
            Some(n) => format!("{}", n),
            None => "no path from zero height found".to_string(),
        };
    println!("part 2: {}", part2);

    std::process::ExitCode::SUCCESS
}
