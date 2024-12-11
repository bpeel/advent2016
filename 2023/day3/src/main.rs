mod util;

use std::process::ExitCode;
use util::Grid;

struct Numbers<'a> {
    grid: &'a Grid,
    next_pos: usize,
}

impl<'a> Iterator for Numbers<'a> {
    type Item = (usize, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
	while self.next_pos < self.grid.values.len() {
	    if !self.grid.values[self.next_pos].is_ascii_digit() {
		self.next_pos += 1;
		continue;
	    }

	    let start = self.next_pos;
	    let row_end = (start / self.grid.width + 1) * self.grid.width;
	    let num_end = (start + 1..row_end).take_while(|&pos| self.grid.values[pos].is_ascii_digit()).last().unwrap_or(start) + 1;
	    self.next_pos = num_end;

	    return Some((start, &self.grid.values[start..num_end]));
	}
	
	None
    }
}

impl<'a> Numbers<'a> {
    fn new(grid: &'a Grid) -> Numbers<'a> {
	Numbers {
	    grid,
	    next_pos: 0,
	}
    }
}

fn has_symbol<I>(
    grid: &Grid,
    positions: I,
) -> bool
where I: Iterator<Item = usize>
{
    for pos in positions {
	let x = (pos % grid.width) as i32;
	let y = (pos / grid.width) as i32;

	for ox in -1..=1 {
	    for oy in -1..=1 {
		if ox == 0 && oy == 0 {
		    continue;
		}

		if let Some(ch) = grid.get((x + ox, y + oy)) {
		    if !ch.is_ascii_digit() && ch != b'.' {
			return true;
		    }
		}
	    }
	}
    }

    false
}

fn main() -> ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let part1 = Numbers::new(&grid)
	.filter(|&(pos, slice)| has_symbol(&grid, pos..pos + slice.len()))
	.map(|(_, slice)| std::str::from_utf8(slice).unwrap().parse::<u64>().unwrap())
	.sum::<u64>();

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
