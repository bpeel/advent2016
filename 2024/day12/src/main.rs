mod bitset;
mod util;
mod walker;

use std::process::ExitCode;
use util::Grid;
use bitset::BitSet;
use walker::VisitResult;

struct Regions<'a> {
    grid: &'a Grid,
    pos: usize,
    visited: BitSet,
}

impl<'a> Iterator for Regions<'a> {
    type Item = BitSet;

    fn next(&mut self) -> Option<BitSet> {
	while self.pos < self.grid.values.len() {
	    if !self.visited.contains(self.pos) {
		let region = fill_region(self.grid, self.pos);
		self.visited.merge(&region);
		return Some(region);
	    }

	    self.pos += 1;
	}

	None
    }
}

impl<'a> Regions<'a> {
    fn new(grid: &Grid) -> Regions {
	Regions {
	    grid,
	    visited: BitSet::new(),
	    pos: 0,
	}
    }
}

fn fill_region(grid: &Grid, pos: usize) -> BitSet {
    let mut region = BitSet::new();
    let start_pos = (
	(pos % grid.width) as i32,
	(pos / grid.width) as i32,
    );
    let letter = grid.values[pos];

    walker::walk::<walker::QuadDirection, _>(start_pos, |_path, pos| {
	if grid.get(pos) != Some(letter) {
	    return VisitResult::Backtrack;
	};

	let pos = (pos.1 as usize * grid.width) + pos.0 as usize;

	if region.contains(pos) {
	    return VisitResult::Backtrack;
	}

	region.set(pos);

	VisitResult::Continue
    });

    region
}

fn perimeter(grid_width: usize, region: &BitSet) -> u32 {
    region.bits().map(|pos| {
	let x = pos % grid_width;

	[
	    x == 0 || !region.contains(pos - 1),
	    x + 1 >= grid_width || !region.contains(pos + 1),
	    pos < grid_width || !region.contains(pos - grid_width),
	    !region.contains(pos + grid_width),
	].into_iter().map(|b| b as u32).sum::<u32>()
    }).sum::<u32>()
}

fn main() -> ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let part1 = Regions::new(&grid)
	.map(|region| {
	    let area = region.len();
	    let perimeter = perimeter(grid.width, &region);
	    area * perimeter as usize
	}).sum::<usize>();

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
