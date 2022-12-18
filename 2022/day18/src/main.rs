use std::collections::HashSet;

mod walker;

use walker::Direction;
use walker::VisitResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Rock {
    x: i8,
    y: i8,
    z: i8,
}

#[derive(Debug, Clone, Copy)]
struct CubeDirection {
    x: i8,
    y: i8,
    z: i8,
}

#[derive(Debug, Clone)]
struct Bounds {
    min: Rock,
    max: Rock,
}

impl CubeDirection {
    fn get_mut(&mut self, axis: usize) -> &mut i8 {
        match axis {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!(),
        }
    }
}

impl Direction for CubeDirection {
    type Pos = Rock;

    fn first_direction() -> CubeDirection {
        CubeDirection { x: -1, y: 0, z: 0 }
    }

    fn next_direction(mut self) -> Option<CubeDirection> {
        for i in 0..3 {
            let part = self.get_mut(i);

            match *part {
                -1 => {
                    *part = 1;
                    return Some(self);
                },
                0 => (),
                1 => {
                    return if i >= 2 {
                        None
                    } else {
                        *part = 0;
                        *self.get_mut(i + 1) = -1;
                        Some(self)
                    };
                },

                _ => unreachable!(),
            };
        }

        unreachable!();
    }

    fn move_pos(self, pos: Rock) -> Rock {
        Rock { x: pos.x + self.x, y: pos.y + self.y, z: pos.z + self.z }
    }
}

fn read_rocks<I>(lines: &mut I) -> Result<HashSet<Rock>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^(-?\d+),(-?\d+),(-?\d+)$").unwrap();
    let mut rocks = HashSet::<Rock>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line {}: invalid syntax",
                                       line_num + 1)),
        };

        let mut parts = [0i8; 3];

        for i in 0..parts.len() {
            parts[i] = match captures[i + 1].parse() {
                Ok(n) => n,
                Err(e) => return Err(format!("line {}: {}", line_num + 1, e)),
            };

            if parts[i] <= i8::MIN + 1 || parts[i] >= i8::MAX - 1 {
                return Err(format!("line {}: coordinate too extreme",
                                   line_num + 1));
            }
        }

        if !rocks.insert(Rock { x: parts[0], y: parts[1], z: parts[2] }) {
            return Err(format!("line {}: duplicate rock", line_num + 1));
        }
    }

    Ok(rocks)
}

fn count_covered_sides_for_rock(rocks: &HashSet<Rock>, rock: &Rock) -> usize {
    (-1..=1).step_by(2).map(|offset| {
        rocks.contains(&Rock { x: rock.x + offset, y: rock.y, z: rock.z })
            as usize +
            rocks.contains(&Rock { x: rock.x, y: rock.y + offset, z: rock.z })
            as usize +
            rocks.contains(&Rock { x: rock.x, y: rock.y, z: rock.z + offset })
            as usize
    }).sum()
}

fn count_covered_sides(rocks: &HashSet<Rock>) -> usize {
    rocks.iter().map(|rock| count_covered_sides_for_rock(rocks, rock)).sum()
}

fn get_bounds(rocks: &HashSet<Rock>) -> Bounds {
    let mut bounds = Bounds {
        min: Rock { x: i8::MAX, y: i8::MAX, z: i8::MAX },
        max: Rock { x: i8::MIN, y: i8::MIN, z: i8::MIN },
    };

    for rock in rocks.iter() {
        bounds.min.x = std::cmp::min(bounds.min.x, rock.x);
        bounds.min.y = std::cmp::min(bounds.min.y, rock.y);
        bounds.min.z = std::cmp::min(bounds.min.z, rock.z);
        bounds.max.x = std::cmp::max(bounds.max.x, rock.x);
        bounds.max.y = std::cmp::max(bounds.max.y, rock.y);
        bounds.max.z = std::cmp::max(bounds.max.z, rock.z);
    }

    bounds
}

fn get_reachable_squares(rocks: &HashSet<Rock>) -> HashSet<Rock> {
    let mut reachable = HashSet::<Rock>::new();
    let bounds = get_bounds(rocks);
    let start_point = Rock {
        x: bounds.min.x - 1,
        y: bounds.min.y - 1,
        z: bounds.min.z - 1,
    };

    walker::walk::<CubeDirection, _>(start_point, |_, pos| {
        if pos.x < bounds.min.x - 1 ||
            pos.x > bounds.max.x + 1 ||
            pos.y < bounds.min.y - 1 ||
            pos.y > bounds.max.y + 1 ||
            pos.z < bounds.min.z - 1 ||
            pos.z > bounds.max.z + 1 {
            return VisitResult::Backtrack;
        }

        if rocks.contains(&pos) {
            return VisitResult::Backtrack;
        }

        if !reachable.insert(pos) {
            return VisitResult::Backtrack;
        }

        VisitResult::Continue
    });

    reachable
}

fn count_reachable_sides_for_rock(reachable: &HashSet<Rock>,
                                  rock: &Rock) -> usize {
    (-1..=1).step_by(2).map(|offset| {
        reachable.contains(&Rock {
            x: rock.x + offset,
            y: rock.y,
            z: rock.z
        }) as usize +
            reachable.contains(&Rock {
                x: rock.x,
                y: rock.y + offset,
                z: rock.z }) as usize +
            reachable.contains(&Rock {
                x: rock.x,
                y: rock.y,
                z: rock.z + offset
            }) as usize
    }).sum()
}

fn count_reachable_sides(rocks: &HashSet<Rock>) -> usize {
    let reachable = get_reachable_squares(rocks);

    rocks.iter().map(|rock| {
        count_reachable_sides_for_rock(&reachable, rock)
    }).sum()
}

fn main() -> std::process::ExitCode {
    let rocks = match read_rocks(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(rocks) => rocks,
    };

    println!("part 1: {}", rocks.len() * 6 - count_covered_sides(&rocks));
    println!("part 2: {}", count_reachable_sides(&rocks));

    std::process::ExitCode::SUCCESS
}
