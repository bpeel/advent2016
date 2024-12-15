mod util;
mod walker;

use std::io::stdin;
use std::process::ExitCode;
use util::Grid;
use walker::QuadDirection;

struct Scene {
    pos: (i32, i32),
    grid: Grid,
}

impl Scene {
    fn new(mut grid: Grid) -> Scene {
        let pos = match grid.values.iter().position(|&ch| ch == b'@') {
            Some(index) => {
                grid.values[index] = b'.';
                (
                    (index % grid.width) as i32,
                    (index / grid.width) as i32,
                )
            },
            None => (0, 0),
        };

        Scene {
            pos,
            grid,
        }
    }

    fn can_move(&self, dir: QuadDirection) -> bool {
        let mut pos = self.pos;

        loop {
            pos = dir.move_pos(pos);

            match self.grid.get(pos) {
                Some(b'.') => break true,
                Some(b'O') => (),
                _ => break false,
            }
        }
    }

    fn step(&mut self, dir: QuadDirection) {
        if self.can_move(dir) {
            let mut pos = dir.move_pos(self.pos);
            self.pos = pos;

            if self.grid.get(pos).unwrap() == b'O' {
                self.grid.values[
                    pos.1 as usize * self.grid.width +
                        pos.0 as usize
                ] = b'.';

                loop {
                    pos = dir.move_pos(pos);

                    if self.grid.get(pos).unwrap() == b'.' {
                        break;
                    }
                }

                self.grid.values[
                    pos.1 as usize * self.grid.width +
                        pos.0 as usize
                ] = b'O';
            }
        }
    }
}

fn read_directions() -> Result<Vec<QuadDirection>, String> {
    let mut dirs = Vec::new();

    for result in stdin().lines() {
        let line = result.map_err(|e| e.to_string())?;

        for ch in line.chars() {
            match QuadDirection::from_char(ch) {
                Some(dir) => dirs.push(dir),
                None => return Err(format!("bad dir: {}", ch)),
            }
        }
    }

    Ok(dirs)
}

fn main() -> ExitCode {
    let mut scene = match Grid::load(&mut stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => Scene::new(grid),
    };

    let dirs = match read_directions() {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(dirs) => dirs,
    };

    for dir in dirs {
        scene.step(dir);
    }

    let part1 = scene.grid.values.iter().enumerate()
        .filter_map(|(index, &ch)| {
            (ch == b'O').then(|| {
                index / scene.grid.width * 100 + index % scene.grid.width
            })
        }).sum::<usize>();

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
