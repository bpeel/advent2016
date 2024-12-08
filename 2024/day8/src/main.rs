mod util;

use std::process::ExitCode;
use util::Grid;
use std::collections::{HashMap, HashSet};

type AntennaMap = HashMap<u8, Vec<(i32, i32)>>;

fn find_antennas(grid: &Grid) -> AntennaMap {
    let mut antennas = AntennaMap::new();

    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            if let Some(ch) = grid.get((x, y)) {
                if (ch as char).is_alphanumeric() {
                    antennas.entry(ch)
                        .and_modify(|locations| locations.push((x, y)))
                        .or_insert_with(|| vec![(x, y)]);
                }
            }
        }
    }

    antennas
}

fn find_antinodes(antennas: &AntennaMap) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();

    for antennas in antennas.values() {
        for (pos, a) in antennas.iter().enumerate() {
            for b in &antennas[pos + 1..] {
                antinodes.insert((a.0 * 2 - b.0, (a.1 * 2 - b.1)));
                antinodes.insert((b.0 * 2 - a.0, (b.1 * 2 - a.1)));
            }
        }
    }

    antinodes
}

fn print_antinodes<'a, I>(
    grid: &Grid,
    antinodes: I,
)
    where I: IntoIterator<Item = &'a (i32, i32)>
{
    let mut grid = grid.clone();

    for &(x, y) in antinodes.into_iter() {
        grid.values[y as usize * grid.width + x as usize] = b'#';
    }

    println!("{}", grid);
}

fn main() -> std::process::ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let antennas = find_antennas(&grid);
    let mut antinodes = find_antinodes(&antennas);

    antinodes.retain(|&pos| grid.get(pos).is_some());

    print_antinodes(&grid, &antinodes);

    println!("Part 1: {}", antinodes.len());

    ExitCode::SUCCESS
}
