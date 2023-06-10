use std::str::FromStr;
use std::process::ExitCode;
use std::fmt;
use std::collections::HashSet;

const N_ORIENTATIONS: usize = 24;
const MIN_MATCHES: usize = 12;

static TRANSFORMATIONS: [[i32; 9]; N_ORIENTATIONS] = [
    [-1, 0, 0, 0, -1, 0, 0, 0, 1],
    [-1, 0, 0, 0, 0, -1, 0, -1, 0],
    [-1, 0, 0, 0, 0, 1, 0, 1, 0],
    [-1, 0, 0, 0, 1, 0, 0, 0, -1],
    [0, -1, 0, -1, 0, 0, 0, 0, -1],
    [0, -1, 0, 0, 0, -1, 1, 0, 0],
    [0, -1, 0, 0, 0, 1, -1, 0, 0],
    [0, -1, 0, 1, 0, 0, 0, 0, 1],
    [0, 0, -1, -1, 0, 0, 0, 1, 0],
    [0, 0, -1, 0, -1, 0, -1, 0, 0],
    [0, 0, -1, 0, 1, 0, 1, 0, 0],
    [0, 0, -1, 1, 0, 0, 0, -1, 0],
    [0, 0, 1, -1, 0, 0, 0, -1, 0],
    [0, 0, 1, 0, -1, 0, 1, 0, 0],
    [0, 0, 1, 0, 1, 0, -1, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 1, 0],
    [0, 1, 0, -1, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, -1, -1, 0, 0],
    [0, 1, 0, 0, 0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0, 0, 0, 0, -1],
    [1, 0, 0, 0, -1, 0, 0, 0, -1],
    [1, 0, 0, 0, 0, -1, 0, 1, 0],
    [1, 0, 0, 0, 0, 1, 0, -1, 0],
    [1, 0, 0, 0, 1, 0, 0, 0, 1],
];

#[derive(PartialEq, Eq, Hash)]
struct Position {
    coords: [i32; 3],
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Position, String> {
        let mut coords = [0; 3];
        let mut parts = s.split(',');

        for coord in coords.iter_mut() {
            let Some(part) = parts.next()
            else {
                return Err("Not enough coordinates".to_string());
            };

            *coord = match part.parse::<i32>() {
                Ok(coord) => coord,
                Err(e) => return Err(e.to_string()),
            };
        }

        if parts.next().is_some() {
            return Err("Too many coords".to_string());
        }

        Ok(Position { coords })
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.coords[0],
            self.coords[1],
            self.coords[2],
        )
    }
}

impl Position {
    fn orientate(&self, orientation: usize) -> Position {
        let x = self.coords[0];
        let y = self.coords[1];
        let z = self.coords[2];
        let m = &TRANSFORMATIONS[orientation];

        Position {
            coords: [
                m[0] * x + m[3] * y + m[6] * z,
                m[1] * x + m[4] * y + m[7] * z,
                m[2] * x + m[5] * y + m[8] * z,
            ],
        }
    }

    fn offset(&self, offsets: &[i32; 3]) -> Position {
        Position {
            coords: [
                self.coords[0] + offsets[0],
                self.coords[1] + offsets[1],
                self.coords[2] + offsets[2],
            ],
        }
    }
}

struct Scanner {
    name: usize,
    beacons: HashSet<Position>,
}

impl Scanner {
    fn new(name: usize) -> Scanner {
        Scanner {
            name,
            beacons: HashSet::new(),
        }
    }

    fn matches_for_orientation_and_offset(
        &self,
        other: &Scanner,
        orientation: usize,
        offset: [i32; 3],
    ) -> bool {
        let mut n_matches = 0;

        for beacon in other.beacons.iter() {
            let pos = beacon.orientate(orientation).offset(&offset);

            if self.beacons.contains(&pos) {
                n_matches += 1;

                if n_matches >= MIN_MATCHES {
                    return true;
                }
            }
        }

        false
    }

    fn matches_for_orientation(
        &self,
        other: &Scanner,
        orientation: usize,
    ) -> Option<[i32; 3]> {
        // Try every pair of beacons as an offset
        for a in self.beacons.iter() {
            for b in other.beacons.iter() {
                let b = b.orientate(orientation);

                let offset = [
                    a.coords[0] - b.coords[0],
                    a.coords[1] - b.coords[1],
                    a.coords[2] - b.coords[2],
                ];

                if self.matches_for_orientation_and_offset(
                    other,
                    orientation,
                    offset.clone(),
                ) {
                    return Some(offset);
                }
            }
        }

        None
    }

    fn matches(&self, other: &Scanner) -> Option<(usize, [i32; 3])> {
        for orientation in 0..N_ORIENTATIONS {
            if let Some(offset) =
                self.matches_for_orientation(other, orientation)
            {
                return Some((orientation, offset));
            }
        }

        None
    }

    fn normalise(&mut self, orientation: usize, offset: [i32; 3]) {
        let mut beacons = HashSet::new();

        for beacon in self.beacons.iter() {
            let pos = beacon.orientate(orientation).offset(&offset);
            beacons.insert(pos);
        }

        self.beacons = beacons;
    }
}

fn read_scanners() -> Result<Vec<Scanner>, String> {
    let mut scanners = Vec::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(e.to_string()),
        };

        if line.starts_with("---") {
            scanners.push(Scanner::new(scanners.len()));
        } else if !line.is_empty() {
            let Some(scanner) = scanners.last_mut()
            else {
                return Err("Missing scanner header".to_string());
            };

            match line.parse::<Position>() {
                Ok(pos) => {
                    scanner.beacons.insert(pos);
                },
                Err(e) => {
                    return Err(format!(
                        "line {}: {}",
                        line_num + 1,
                        e,
                    ));
                },
            }
        }
    }

    Ok(scanners)
}

fn biggest_distance(offsets: &[[i32; 3]]) -> i32 {
    let mut biggest = 0;

    for a in offsets.iter() {
        for b in offsets.iter() {
            let distance = (0..3).map(|i| a[i].abs_diff(b[i]) as i32).sum();

            if distance > biggest {
                biggest = distance;
            }
        }
    }

    biggest
}

fn main() -> ExitCode {
    let mut scanners = match read_scanners() {
        Ok(scanners) => scanners,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut matched_scanners = vec![scanners.swap_remove(0)];
    let mut scanner_offsets = vec![[0, 0, 0]];

    'match_loop: loop {
        for (scanner_num, scanner) in scanners.iter().enumerate() {
            for normalised in matched_scanners.iter() {
                if let Some((orientation, offset)) =
                    normalised.matches(scanner)
                {
                    let mut scanner = scanners.swap_remove(scanner_num);

                    println!(
                        "matched {} to {} with orientation {} and offset {:?}",
                        scanner.name,
                        normalised.name,
                        orientation,
                        offset,
                    );

                    scanner_offsets.push(offset.clone());

                    scanner.normalise(orientation, offset);

                    matched_scanners.push(scanner);

                    continue 'match_loop;
                }
            }
        }

        break;
    }

    if !scanners.is_empty() {
        println!("!!! unmatched beacons: {} !!!", scanners.len());
    }

    let mut unique_beacons = HashSet::new();

    for scanner in matched_scanners.into_iter() {
        unique_beacons.extend(scanner.beacons.into_iter());
    }

    println!("part 1: {} unique beacons", unique_beacons.len());
    println!("part 2: {}", biggest_distance(&scanner_offsets));

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_orientations_different() {
        const BASE_POS: Position = Position { coords: [3, 4, 5] };

        let mut orientations = HashSet::new();

        for i in 0..N_ORIENTATIONS {
            orientations.insert(BASE_POS.orientate(i));
        }

        assert_eq!(orientations.len(), N_ORIENTATIONS);
    }
}
