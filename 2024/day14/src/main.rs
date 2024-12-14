use std::process::ExitCode;
use std::sync::LazyLock;
use std::str::FromStr;
use std::fmt;
use std::collections::HashSet;

static ROBOT_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap()
});

static BLOCKS: [char; 16] = [
    ' ',
    '▘',
    '▝',
    '▀',
    '▖',
    '▌',
    '▞',
    '▛',
    '▗',
    '▚',
    '▐',
    '▜',
    '▄',
    '▙',
    '▟',
    '█',
];

struct Scene {
    grid_size: (i32, i32),
    robots: Vec<Robot>,
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text_w = (self.grid_size.0 as usize + 1) / 2;
        let text_h = (self.grid_size.1 as usize + 1) / 2;

        let mut grid = vec![0u8; text_w * text_h];

        for robot in self.robots.iter() {
            let pos = robot.pos.1 as usize / 2 * text_w +
                robot.pos.0 as usize / 2;
            let bit = (robot.pos.0 & 1) |
                ((robot.pos.1 & 1) << 1);

            grid[pos] |= 1 << bit;
        }

        for y in 0..text_h {
            for x in 0..text_w {
                let block = grid[y * text_w + x] as usize;
                write!(f, "{}", BLOCKS[block])?;
            }

            if y + 1 < text_h {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl Scene {
    fn step(&mut self, count: i32){
        for robot in self.robots.iter_mut() {
            robot.step(self.grid_size, count);
        }
    }
}

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

fn count_diagnols(grid_size: (i32, i32), grid: &HashSet<(i32, i32)>) -> usize {
    let mut count = 0;

    for y in 0..grid_size.1 - 1 {
        for x in 0..grid_size.0 - 1 {
            let bits = (grid.contains(&(x, y)) as u8) |
            ((grid.contains(&(x + 1, y)) as u8) << 1) |
            ((grid.contains(&(x, y + 1)) as u8) << 2) |
            ((grid.contains(&(x + 1, y + 1)) as u8) << 3);

            if bits == 9 || bits == 6 {
                count += 1;
            }
        }
    }

    count
}

fn part2(mut scene: Scene) -> usize {
    let mut grid = HashSet::new();
    let min_diagonols = scene.grid_size.1 as usize * 6 / 4;

    for i in 1.. {
        scene.step(1);

        grid.clear();

        for robot in scene.robots.iter() {
            grid.insert(robot.pos.clone());
        }

        let n_diagonols = count_diagnols(scene.grid_size, &grid);

        if n_diagonols >= min_diagonols {
            println!("{}", scene);
            return i;
        }
    }

    unreachable!("range expired!");
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

    let scene = Scene {
        grid_size,
        robots,
    };

    println!("{}", part2(scene));

    ExitCode::SUCCESS
}
