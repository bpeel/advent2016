mod util;
mod walker;

use std::io::BufRead;
use std::process::ExitCode;
use util::Grid;

#[derive(Debug, Clone)]
struct Item {
    start: u32,
    end: u32,
}

fn read_items<I>(lines: &mut I) -> Result<Vec<Item>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^(\d+),(\d+)$").unwrap();

    lines.enumerate().map(|(line_num, result)| {
        let line = result.map_err(|e| e.to_string())?;

        let captures = re.captures(&line).ok_or_else(|| {
            format!("line: {}: invalid syntax", line_num + 1)
        })?;

        Ok(Item {
            start: captures[1].parse::<u32>().unwrap(),
            end: captures[2].parse::<u32>().unwrap(),
        })
    }).collect()
}

fn main() -> ExitCode {
    let grid;
    let items;

    {
        let mut input = std::io::stdin().lock();

        grid = match Grid::load(&mut input) {
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
            Ok(grid) => grid,
        };

        items = match read_items(&mut input.lines()) {
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
            Ok(items) => items,
        };
    }

    println!("{}", grid);
    println!("{:?}", items);

    walker::shortest_walk::<walker::QuadDirection, _>((0, 0), |path, pos| {
        if pos == (10, 10) {
            println!("{} {:?}", path.len(), path);
            return walker::VisitResult::Goal;
        }

        match grid.get(pos) {
            Some(b'.') => return walker::VisitResult::Continue,
            None | Some(_) => return walker::VisitResult::Backtrack,
        }
    });

    ExitCode::SUCCESS
}
