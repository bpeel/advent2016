use std::process::ExitCode;
use std::sync::LazyLock;
use std::str::FromStr;

static ROBOT_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap()
});

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),
    v: (i32, i32),
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Robot, String> {
        let Some(captures) = ROBOT_RE.captures(s)
        else {
            return Err("bad robot".to_string());
        };

        let mut parts: [i32; 4] = Default::default();

        for (i, p) in parts.iter_mut().enumerate() {
            let Ok(v) = captures[i + 1].parse::<i32>()
            else {
                return Err("bad number".to_string());
            };

            *p = v;
        }

        Ok(Robot {
            pos: (parts[0], parts[1]),
            v: (parts[2], parts[3]),
        })
    }
}

impl Robot {
    fn step(&mut self, grid_size: (i32, i32), count: i32) {
        self.pos.0 = (self.pos.0 + self.v.0 * count).rem_euclid(grid_size.0);
        self.pos.1 = (self.pos.1 + self.v.1 * count).rem_euclid(grid_size.1);
    }
}

fn read_robots<I>(lines: &mut I) -> Result<Vec<Robot>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut robots = Vec::<Robot>::new();

    for (line_num, result) in lines.enumerate() {
        let line = result.map_err(|e| e.to_string())?;

        robots.push(
            line.parse::<Robot>()
                .map_err(|e| {
                    format!(
                        "line: {}: {}",
                        line_num + 1,
                        e,
                    )
                })?
        );
    }

    Ok(robots)
}

fn grid_size() -> Result<(i32, i32), String> {
    let mut parts = [101, 103];

    for (i, arg) in std::env::args().skip(1).take(2).enumerate() {
        let Ok(v) = arg.parse::<i32>()
        else {
            return Err(format!("bad number: {}", arg));
        };

        parts[i] = v;
    }

    Ok((parts[0], parts[1]))
}

fn part1(grid_size: (i32, i32), robots: &[Robot]) -> usize {
    let mut quadrants = [0; 4];
    let mid = (grid_size.0 / 2, grid_size.1 / 2);

    for robot in robots.iter() {
        let mut robot = robot.clone();

        robot.step(grid_size, 100);

        if robot.pos.0 == mid.0 || robot.pos.1 == mid.1 {
            continue;
        }

        let qx = (robot.pos.0 >= mid.0) as usize;
        let qy = (robot.pos.1 >= mid.1) as usize;

        quadrants[qy * 2 + qx] += 1;
    }

    quadrants.iter().product()
}

fn main() -> ExitCode {
    let grid_size = match grid_size() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let robots = match read_robots(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(robots) => robots,
    };

    println!("{:?}", part1(grid_size, &robots));

    ExitCode::SUCCESS
}
