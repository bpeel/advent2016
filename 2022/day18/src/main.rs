use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Rock {
    x: i8,
    y: i8,
    z: i8,
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

            if parts[i] <= i8::MIN || parts[i] >= i8::MAX {
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

fn main() -> std::process::ExitCode {
    let rocks = match read_rocks(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(rocks) => rocks,
    };

    println!("part 1: {}", rocks.len() * 6 - count_covered_sides(&rocks));

    std::process::ExitCode::SUCCESS
}
