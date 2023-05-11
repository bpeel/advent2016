use std::str::FromStr;
use std::cmp;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for TargetArea {
    type Err = String;

    fn from_str(s: &str) -> Result<TargetArea, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^target area: \
                                               x=(?<xmin>-?\\d+)\\.\\.\
                                               (?<xmax>-?\\d+), \
                                               y=(?<ymin>-?\\d+)\\.\\.\
                                               (?<ymax>-?\\d+)$").unwrap();
        }

        let Some(captures) = RE.captures(s)
        else { return Err("invalid target area description".to_string()) };

        fn parse_item(s: &str) -> Result<i32, String> {
            s.parse::<i32>().map_err(|e| e.to_string())
        }

        Ok(TargetArea {
            x_min: parse_item(&captures["xmin"])?,
            x_max: parse_item(&captures["xmax"])?,
            y_min: parse_item(&captures["ymin"])?,
            y_max: parse_item(&captures["ymax"])?,
        })
    }
}

impl TargetArea {
    fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x_min && x <= self.x_max
            && y >= self.y_min && y <= self.y_max
    }
}

fn max_height_with_target(
    mut vx: i32,
    mut vy: i32,
    ta: &TargetArea,
) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    loop {
        if y > max_y {
            max_y = y;
        }

        if ta.contains(x, y) {
            break Some(max_y);
        }

        if x > ta.x_max || y < ta.y_min {
            break None;
        }

        x += vx;
        y += vy;

        vx = cmp::max(0, vx - 1);
        vy -= 1;
    }
}

fn part1(ta: &TargetArea) -> Option<i32> {
    let mut max_height = None;

    for vx in 1..=(ta.x_max + 1) {
        for vy in 0..=(ta.y_min.abs() + 1) {
            if let Some(height) = max_height_with_target(vx, vy, ta) {
                max_height = Some(max_height.map_or(
                    height,
                    |old_height| cmp::max(height, old_height),
                ));
            }
        }
    }

    max_height
}

fn process_lines<I>(lines: I) -> std::process::ExitCode
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut ret = std::process::ExitCode::SUCCESS;

    for (line_num, line) in lines.enumerate() {
        match line {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(line) => {
                match line.parse::<TargetArea>() {
                    Err(e) => {
                        eprintln!("line {}: {}", line_num + 1, e);
                        ret = std::process::ExitCode::FAILURE;
                    },
                    Ok(ta) => {
                        println!(
                            "part 1: {:?}",
                            part1(&ta),
                        );
                    },
                };
            },
        }
    }

    ret
}

fn main() -> std::process::ExitCode {
    let mut args = std::env::args();

    args.next();

    if args.len() > 0 {
        process_lines(args.map(|arg| Ok(arg)))
    } else {
        process_lines(std::io::stdin().lines())
    }
}
