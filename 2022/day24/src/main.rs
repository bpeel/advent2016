mod walker;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use walker::{QuadDirection, VisitResult};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum BlizzardDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Blizzard {
    x: usize,
    y: usize,
    direction: BlizzardDirection,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: (i32, i32),
    blizzard_pos: usize,
}

impl State {
    fn new(pos: (i32, i32), blizzard_pos: usize) -> State {
        State { pos, blizzard_pos }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    lcm: usize,
    start_pos: usize,
    end_pos: usize,
    blizzards: Vec<Blizzard>,
}

impl Grid {
    fn blizzard_pos(&self, blizzard: &Blizzard, minute: usize) -> (usize, usize) {
        match blizzard.direction {
            BlizzardDirection::Up => (blizzard.x, (blizzard.y + (self.height - 1) * minute) % self.height),
            BlizzardDirection::Down => (blizzard.x, (blizzard.y + minute) % self.height),
            BlizzardDirection::Left => ((blizzard.x + (self.width - 1) * minute) % self.width, blizzard.y),
            BlizzardDirection::Right => ((blizzard.x + minute) % self.width, blizzard.y),
        }
    }

    fn occupied(&self, minute: usize, pos: (usize, usize)) -> bool {
        for blizzard in self.blizzards.iter() {
            if self.blizzard_pos(blizzard, minute) == pos {
                return true;
            }
        }

        return false;
    }
}

fn lcm(a: usize, b: usize) -> usize {
    for i in std::cmp::min(a, b)..=a * b {
        if i % a == 0 && i % b == 0 {
            return i;
        }
    }

    panic!("no lcm?");
}

fn read_grid<I>(lines: &mut I) -> Result<Grid, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut grid = Grid {
        width: usize::MAX,
        height: usize::MAX,
        lcm: 0,
        start_pos: 0,
        end_pos: 0,
        blizzards: Vec::new(),
    };

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        if line_num == 0 {
            if line.len() < 2 {
                return Err("invalid first line".to_string());
            }
            grid.width = line.len() - 2;
            grid.start_pos = match line.find(".") {
                Some(n) if n > 0 => n - 1,
                _ => return Err("invalid first line".to_string()),
            };
            continue;
        }

        if let Some(_) = line.find("##") {
            grid.end_pos = match line.find(".") {
                Some(n) if n > 0 => n - 1,
                _ => return Err("invalid last line".to_string()),
            };
            grid.height = line_num - 1;
            break;
        }

        if line.len() != grid.width + 2 {
            return Err(format!("invalid line {}", line_num + 1));
        }

        let y = line_num - 1;

        for (x, ch) in line.as_bytes()[1..line.len() - 1].iter().enumerate() {
            let direction = match ch {
                b'<' => BlizzardDirection::Left,
                b'>' => BlizzardDirection::Right,
                b'^' => BlizzardDirection::Up,
                b'v' => BlizzardDirection::Down,
                _ => continue,
            };

            grid.blizzards.push(Blizzard {
                x, y, direction
            });
        }
    }

    if grid.width == usize::MAX || grid.height == usize::MAX {
        return Err("missing start or end line".to_string());
    }

    grid.lcm = lcm(grid.width, grid.height);

    Ok(grid)
}

fn solve(grid: &Grid) -> usize {
    let mut visited = HashMap::<State, usize>::new();
    let mut best = usize::MAX;

    walker::walk::<QuadDirection, _>((grid.start_pos as i32, -1), |path, pos| {
        if pos.0 < 0 || pos.0 as usize >= grid.width {
            return VisitResult::Backtrack;
        }

        if pos.1 == -1 {
            if pos.0 as usize != grid.start_pos {
                return VisitResult::Backtrack;
            }
        } else if pos.1 as usize == grid.height {
            if pos.0 as usize != grid.end_pos {
                return VisitResult::Backtrack;
            }
        } else if pos.1 < 0 || pos.1 as usize >= grid.height {
            return VisitResult::Backtrack;
        } else if grid.occupied(path.len(), (pos.0 as usize, pos.1 as usize)) {
            return VisitResult::Backtrack;
        }

        let state = State::new(pos, path.len() % grid.lcm);

        match visited.entry(state) {
            Entry::Occupied(mut e) => {
                if *e.get() <= path.len() {
                    return VisitResult::Backtrack;
                } else {
                    e.insert(path.len());
                }
            },
            Entry::Vacant(mut e) => {
                e.insert(path.len());
            },
        }

        if pos.1 >= 0 && pos.1 as usize == grid.height {
            if path.len() < best {
                best = path.len();
                println!("{}", path.len());
            }
            return VisitResult::Backtrack;
        }

        VisitResult::Continue
    });

    best
}

fn main() -> std::process::ExitCode {
    let grid = match read_grid(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(items) => items,
    };

    println!("part 1: {}", solve(&grid));

    std::process::ExitCode::SUCCESS
}
