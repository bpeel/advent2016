mod util;
mod walker;

use std::io::BufRead;

#[derive(Debug, Clone)]
struct Item {
    start: u32,
    end: u32,
}

fn read_items<I>(lines: &mut I) -> Result<Vec<Item>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^(\d+),(\d+)$").unwrap();
    let mut items = Vec::<Item>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line: {}: invalid syntax",
                                       line_num + 1)),
        };

        items.push(Item {
            start: captures[1].parse::<u32>().unwrap(),
            end: captures[2].parse::<u32>().unwrap(),
        });
    }

    Ok(items)
}

fn main() -> std::process::ExitCode {
    let grid;
    let items;

    {
        let mut input = std::io::stdin().lock();

        grid = match util::Grid::load(&mut input) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(grid) => grid,
        };

        items = match read_items(&mut input.lines()) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(items) => items,
        };
    }

    println!("{}", grid);
    println!("{:?}", items);

    walker::shortest_walk((0, 0), |path, (x, y)| {
        if x < 0 || y < 0 {
            return walker::VisitResult::BACKTRACK;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= grid.width || y >= grid.height {
            return walker::VisitResult::BACKTRACK;
        }

        if x == 10 && y == 10 {
            println!("{} {:?}", path.len(), path);
            return walker::VisitResult::GOAL;
        }

        if grid.values[y * grid.height + x] != b'.' {
            return walker::VisitResult::BACKTRACK;
        }

        return walker::VisitResult::CONTINUE;
    });

    std::process::ExitCode::SUCCESS
}
