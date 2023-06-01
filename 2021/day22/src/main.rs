use regex::Regex;
use lazy_static::lazy_static;
use std::str::FromStr;
use std::process::ExitCode;

#[derive(Clone, Debug)]
struct CoordRange {
    min: i32,
    max: i32,
}

#[derive(Clone, Debug)]
struct CubeRange {
    coords: [CoordRange; 3]
}

impl CubeRange {
    fn intersects(&self, other: &CubeRange) -> bool {
        for i in 0..self.coords.len() {
            if other.coords[i].max <= self.coords[i].min
                || other.coords[i].min >= self.coords[i].max
            {
                return false;
            }
        }

        true
    }

    fn contains(&self, other: &CubeRange) -> bool {
        for i in 0..self.coords.len() {
            if self.coords[i].min > other.coords[i].min
                || self.coords[i].max < other.coords[i].max
            {
                return false;
            }
        }

        true
    }

    fn count_cubes(&self) -> u32 {
        let mut n_cubes = 1;

        for range in self.coords.iter() {
            n_cubes *= (range.max - range.min) as u32;
        }

        n_cubes
    }
}

impl FromStr for CubeRange {
    type Err = String;

    fn from_str(s: &str) -> Result<CubeRange, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "^x=(-?\\d+)\\.\\.(-?\\d+),\
                 y=(-?\\d+)\\.\\.(-?\\d+),\
                 z=(-?\\d+)\\.\\.(-?\\d+)$"
            ).unwrap();
        }

        let Some(captures) = RE.captures(s)
        else { return Err("Invalid CubeRange".to_string()); };

        let mut nums = [0i32; 3 * 2];

        for (i, num) in nums.iter_mut().enumerate() {
            *num = match captures[i + 1].parse() {
                Ok(n) => n,
                Err(e) => return Err(e.to_string()),
            };
        }

        for i in 0..3 {
            if nums[i * 2] > nums[i * 2 + 1] {
                return Err(format!(
                    "Invalid range {}..{}",
                    nums[i * 2],
                    nums[i * 2 + 1],
                ));
            }
        }

        Ok(CubeRange {
            coords: [
                CoordRange { min: nums[0], max: nums[1] + 1 },
                CoordRange { min: nums[2], max: nums[3] + 1 },
                CoordRange { min: nums[4], max: nums[5] + 1 },
            ],
        })
    }
}

struct CubeList {
    ranges: Vec<CubeRange>,
    temp: Vec<CubeRange>,
}

impl CubeList {
    fn new() -> CubeList {
        CubeList {
            ranges: Vec::new(),
            temp: Vec::new(),
        }
    }

    fn from_cube_range(range: &CubeRange) -> CubeList {
        CubeList {
            ranges: vec![range.clone()],
            temp: Vec::new(),
        }
    }

    fn subtract(&mut self, range: &CubeRange) {
        self.temp.truncate(0);

        for part in self.ranges.iter() {
            // If the range is outside the part then just leave it as is
            if !part.intersects(range) {
                self.temp.push(part.clone());
                continue;
            }

            // If the range covers the entire part then skip it
            if range.contains(part) {
                continue;
            }

            let mut part = part.clone();

            for i in 0..part.coords.len() {
                if range.coords[i].min > part.coords[i].min {
                    let mut to_add = part.clone();
                    to_add.coords[i].max = range.coords[i].min;
                    self.temp.push(to_add);

                    part.coords[i].min = range.coords[i].min;

                    if part.coords[i].min >= part.coords[i].max {
                        break;
                    }
                }

                if range.coords[i].max < part.coords[i].max {
                    let mut to_add = part.clone();
                    to_add.coords[i].min = range.coords[i].max;
                    self.temp.push(to_add);

                    part.coords[i].max = range.coords[i].max;

                    if part.coords[i].min >= part.coords[i].max {
                        break;
                    }
                }
            }
        }

        std::mem::swap(&mut self.ranges, &mut self.temp);
    }

    fn add(&mut self, range: &CubeRange) {
        // Cut the range up so that only ranges that don’t already
        // intersect with self are in it
        let mut range = CubeList::from_cube_range(range);

        for part in self.ranges.iter() {
            range.subtract(part);
        }

        // Anything left in the range can just be added directly to
        // the list of ranges
        self.ranges.extend(range.ranges);
    }

    fn count_cubes(&self) -> u32 {
        self.ranges.iter().map(|range| range.count_cubes()).sum()
    }
}

impl Clone for CubeList {
    fn clone(&self) -> CubeList {
        CubeList {
            ranges: self.ranges.clone(),
            temp: Vec::new(),
        }
    }
}

enum ActionType {
    On,
    Off,
}

struct Action {
    action_type: ActionType,
    range: CubeRange,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Action, String> {
        let (action_type, rest) = if let Some(tail) = s.strip_prefix("on ") {
            (ActionType::On, tail)
        } else if let Some(tail) = s.strip_prefix("off ") {
            (ActionType::Off, tail)
        } else {
            return Err("Invalid action".to_string());
        };

        Ok(Action {
            action_type,
            range: rest.parse()?,
        })
    }
}

fn main() -> ExitCode {
    let mut cube_list = CubeList::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
        };

        let action = match line.parse::<Action>() {
            Ok(action) => action,
            Err(e) => {
                eprintln!("line {}: {}", line_num + 1, e);
                return ExitCode::FAILURE;
            }
        };

        match action.action_type {
            ActionType::On => cube_list.add(&action.range),
            ActionType::Off => cube_list.subtract(&action.range),
        }
    }

    let mut small_region = cube_list.clone();

    for i in 0..3 {
        let all_region = CubeRange {
            coords: [
                CoordRange { min: i32::MIN, max: i32::MAX },
                CoordRange { min: i32::MIN, max: i32::MAX },
                CoordRange { min: i32::MIN, max: i32::MAX },
            ],
        };

        let mut min_region = all_region.clone();
        min_region.coords[i].max = -50;
        small_region.subtract(&min_region);

        let mut max_region = all_region.clone();
        max_region.coords[i].min = 51;
        small_region.subtract(&max_region);
    }

    println!("part 1: {}", small_region.count_cubes());

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersects() {
        let tests = [
            (
                "x=10..12,y=10..12,z=10..12",
                "x=10..12,y=10..12,z=10..12",
                true,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=13..15,y=10..12,z=10..12",
                false,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=7..9,y=10..12,z=10..12",
                false,
            ),
        ];

        for &(a, b, result) in tests.iter() {
            let a = a.parse::<CubeRange>().unwrap();
            let b = b.parse::<CubeRange>().unwrap();
            assert_eq!(a.intersects(&b), result);
        }
    }

    #[test]
    fn contains() {
        let tests = [
            (
                "x=10..12,y=10..12,z=10..12",
                "x=10..12,y=10..12,z=10..12",
                true,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=11..11,y=11..11,z=11..11",
                true,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=9..12,y=10..12,z=10..12",
                false,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=10..12,y=10..12,z=10..13",
                false,
            ),
        ];

        for &(a, b, result) in tests.iter() {
            let a = a.parse::<CubeRange>().unwrap();
            let b = b.parse::<CubeRange>().unwrap();
            assert_eq!(a.contains(&b), result);
        }
    }

    #[test]
    fn subtract() {
        let tests = [
            (
                "x=10..12,y=10..12,z=10..12",
                "x=11..11,y=11..11,z=11..11",
                26,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=11..100,y=11..11,z=11..11",
                25,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=13..15,y=11..11,z=11..11",
                27,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=10..12,y=10..12,z=10..12",
                0,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=9..13,y=8..14,z=7..15",
                0,
            ),
        ];

        for &(a, b, result) in tests.iter() {
            let mut a = CubeList::from_cube_range(
                &a.parse::<CubeRange>().unwrap()
            );
            let b = b.parse::<CubeRange>().unwrap();

            a.subtract(&b);

            assert_eq!(a.count_cubes(), result);
        }
    }

    #[test]
    fn add() {
        let tests = [
            (
                "x=10..12,y=10..12,z=10..12",
                "x=11..11,y=11..11,z=11..11",
                27,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=13..13,y=11..11,z=11..11",
                28,
            ),
            (
                "x=10..12,y=10..12,z=10..12",
                "x=9..13,y=9..13,z=9..13",
                27 + 98,
            ),
        ];

        for &(a, b, result) in tests.iter() {
            let mut a = CubeList::from_cube_range(
                &a.parse::<CubeRange>().unwrap()
            );
            let b = b.parse::<CubeRange>().unwrap();

            a.add(&b);

            assert_eq!(a.count_cubes(), result);
        }
    }
}
