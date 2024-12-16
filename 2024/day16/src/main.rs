mod util;
mod walker;

use std::process::ExitCode;
use util::Grid;
use walker::{Direction, VisitResult};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

type Pos = (u8, (i32, i32));

#[derive(PartialEq, Eq, Clone, Copy)]
enum TurnDirection {
    Left,
    Right,
    Forward,
}

impl Direction for TurnDirection {
    type Pos = Pos;

    fn first_direction() -> TurnDirection {
        TurnDirection::Left
    }

    fn next_direction(self) -> Option<TurnDirection> {
        match self {
            TurnDirection::Left => Some(TurnDirection::Right),
            TurnDirection::Right => Some(TurnDirection::Forward),
            TurnDirection::Forward => None,
        }
    }

    fn move_pos(self, pos: Pos) -> Pos {
        match self {
            TurnDirection::Left => ((pos.0 + 3) % 4, pos.1),
            TurnDirection::Right => ((pos.0 + 1) % 4, pos.1),
            TurnDirection::Forward => {
                let (x, y) = pos.1;

                let xy = match pos.0 {
                    0 => (x + 1, y),
                    1 => (x, y + 1),
                    2 => (x - 1, y),
                    3 => (x, y - 1),
                    _ => unreachable!("bad direction"),
                };

                (pos.0, xy)
            },
        }
    }
}

fn main() -> ExitCode {
    let grid = match Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let Some(start_pos) = grid.values.iter().position(|&ch| {
        ch == b'S'
    }).map(|index| {
        (
            0,
            (
                (index % grid.width) as i32,
                (index / grid.width) as i32,
            ),
        )
    })
    else {
        eprintln!("grid has no start");
        return ExitCode::FAILURE;
    };

    let mut visited = HashMap::new();
    let mut best = None;

    walker::walk::<TurnDirection, _>(start_pos, |path, pos| {
        let Some(ch) = grid.get(pos.1)
        else {
            return VisitResult::Backtrack;
        };

        if ch == b'#' {
            return VisitResult::Backtrack;
        }

        let score = path.iter().map(|&(dir, _pos)| {
            if dir == TurnDirection::Forward {
                1
            } else {
                1000
            }
        }).sum::<u32>();

        match visited.entry(pos) {
            Entry::Occupied(mut e) => {
                if *e.get() <= score {
                    return VisitResult::Backtrack;
                } else {
                    *e.get_mut() = score;
                }
            },
            Entry::Vacant(e) => {
                e.insert(score);
            },
        }

        if ch == b'E' {
            match best {
                Some(old) => {
                    if old > score {
                        best = Some(score);
                    }
                },
                None => best = Some(score),
            }
            VisitResult::Goal
        } else {
            VisitResult::Continue
        }
    });

    print!("Part 1: ");

    match best {
        Some(score) => println!("{}", score),
        None => println!("no route"),
    }

    ExitCode::SUCCESS
}
