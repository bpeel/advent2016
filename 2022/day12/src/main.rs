mod util;
mod walker;

fn get_start(grid: &util::Grid) -> (i32, i32) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.values[y * grid.width + x] == b'E' {
                return (x as i32, y as i32);
            }
        }
    }

    panic!(" no start");
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
    let grid;

    {
        let mut input = std::io::stdin().lock();

        grid = match util::Grid::load(&mut input) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(grid) => grid,
        };
    }

    let distances = walker::shortest_walk(get_start(&grid), |path, (xp, yp)| {
        if xp < 0 || yp < 0 {
            return walker::VisitResult::BACKTRACK;
        }

        let x = xp as usize;
        let y = yp as usize;

        if x >= grid.width || y >= grid.height {
            return walker::VisitResult::BACKTRACK;
        }

        if path.len() > 0 {
            let (last_x, last_y) = path[path.len() - 1]
                .opposite().move_pos((xp, yp));
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

    println!("{}",
             distances
             .iter()
             .filter_map(|(&(x, y), &v)| {
                 let l = grid.values[x as usize + y as usize * grid.width];
                 if letter_height(l) == 0 {
                     Some(v)
                 } else {
                     None
                 }
             }).min().unwrap());

    std::process::ExitCode::SUCCESS
}
