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

fn add_harmonics(
    grid: &Grid,
    mut pos: (i32, i32),
    dir: (i32, i32),
    antinodes: &mut HashSet<(i32, i32)>,
) {
    while grid.get(pos).is_some() {
        antinodes.insert(pos);

        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
}

fn find_antinodes_with_harmonics(
    grid: &Grid,
    antennas: &AntennaMap,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();

    for antennas in antennas.values() {
        for (pos, &a) in antennas.iter().enumerate() {
            for &b in &antennas[pos + 1..] {
                add_harmonics(grid, a, (b.0 - a.0, b.1 - a.1), &mut antinodes);
                add_harmonics(grid, b, (a.0 - b.0, a.1 - b.1), &mut antinodes);
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

    let antinodes_with_harmonics =
        find_antinodes_with_harmonics(&grid, &antennas);

    print_antinodes(&grid, &antinodes_with_harmonics);

    println!("Part 1: {}", antinodes.len());
    println!("Part 2: {}", antinodes_with_harmonics.len());

    ExitCode::SUCCESS
}
