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

struct Number {
    value: u32,
    y: i32,
    x: i32,
    len: i32,
}

fn has_symbol(
    grid: &Grid,
    number: &Number,
) -> bool {
    for y in number.y - 1..=number.y + 1 {
	for x in number.x - 1..=number.x + number.len {
	    if let Some(ch) = grid.get((x, y)) {
		if !ch.is_ascii_digit() && ch != b'.' {
		    return true;
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

    let numbers = Numbers::new(&grid)
	.map(|(pos, slice)| {
	    Number {
		value: std::str::from_utf8(slice).unwrap()
		    .parse::<u32>().unwrap(),
		x: (pos % grid.width) as i32,
		y: (pos / grid.width) as i32,
		len: slice.len() as i32,
	    }
	})
	.collect::<Vec<_>>();

    let part1 = numbers.iter()
	.filter(|&num| has_symbol(&grid, num))
	.map(|num| num.value as u64)
	.sum::<u64>();

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
