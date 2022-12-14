#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LineDirection {
    Down,
    Right,
}

#[derive(Debug, Clone)]
struct Line {
    start: (i32, i32),
    length: i32,
    direction: LineDirection,
}

impl Line {
    fn iter(&self) -> LineIter {
        LineIter::new(self)
    }

    fn end(&self) -> (i32, i32) {
        match self.direction {
            LineDirection::Down =>
                (self.start.0, self.start.1 + self.length - 1),
            LineDirection::Right =>
                (self.start.0 + self.length - 1, self.start.1),
        }
    }
}

#[derive(Debug, Clone)]
struct LineIter<'a> {
    line: &'a Line,
    i: i32,
}

impl<'a> LineIter<'a> {
    fn new(line: &'a Line) -> LineIter<'a> {
        LineIter { line, i: 0 }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.line.length {
            let res = match self.line.direction {
                LineDirection::Down =>
                    (self.line.start.0, self.line.start.1 + self.i),
                LineDirection::Right =>
                    (self.line.start.0 + self.i, self.line.start.1),
            };
            self.i += 1;
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Bounds {
    min: (i32, i32),
    max: (i32, i32),
}

impl Bounds {
    fn new() -> Bounds {
        Bounds {
            min: (i32::MAX, 0),
            max: (i32::MIN, i32::MIN),
        }
    }

    fn add_point(&mut self, (x, y): (i32, i32)) {
        if x < self.min.0 {
            self.min.0 = x;
        }
        if x > self.max.0 {
            self.max.0 = x;
        }
        if y < self.min.1 {
            self.min.1 = y;
        }
        if y > self.max.1 {
            self.max.1 = y;
        }
    }

    fn width(&self) -> usize {
        (self.max.0 - self.min.0 + 1) as usize
    }

    fn height(&self) -> usize {
        (self.max.1 - self.min.1 + 1) as usize
    }

    fn contains(&self, point: (i32, i32)) -> bool {
        point.0 >= self.min.0 &&
            point.0 <= self.max.0 &&
            point.1 >= self.min.1 &&
            point.1 <= self.max.1
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum GridSpace {
    Empty,
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
struct Grid {
    bounds: Bounds,
    values: Vec<GridSpace>,
}

impl Grid {
    fn new(lines: &[Line]) -> Grid {
        let bounds = get_bounds(lines);
        let vec_size = bounds.width() * bounds.height();
        let mut grid = Grid {
            bounds,
            values: vec![GridSpace::Empty; vec_size],
        };

        for line in lines {
            for point in line.iter() {
                let i = grid.values_index(point);
                grid.values[i] = GridSpace::Rock;
            }
        }

        grid
    }

    fn values_index(&self, point: (i32, i32)) -> usize {
        (point.0 - self.bounds.min.0) as usize +
            (point.1 - self.bounds.min.1) as usize *
            self.bounds.width()
    }

    fn get_mut(&mut self, point: (i32, i32)) -> Option<&mut GridSpace> {
        if self.bounds.contains(point) {
            let i = self.values_index(point);
            Some(&mut self.values[i])
        } else {
            None
        }
    }

    fn get(&self, point: (i32, i32)) -> Option<GridSpace> {
        if self.bounds.contains(point) {
            Some(self.values[self.values_index(point)])
        } else {
            None
        }
    }

    fn add_sand(&mut self, start_point: (i32, i32)) -> bool {
        // Sanity check that the start point is a valid empty space
        match self.get(start_point) {
            Some(GridSpace::Empty) => (),
            _ => return false,
        }

        let mut point = start_point;

        'outer: loop {
            for x_offset in [0, -1, 1] {
                let next_point = (point.0 + x_offset, point.1 + 1);

                match self.get(next_point) {
                    None => {
                        // Sand has gone off the edge of the board into the void
                        break 'outer false;
                    },
                    Some(GridSpace::Empty) => {
                        point = next_point;
                        continue 'outer;
                    },

                    _ => (),
                }
            }

            // If we make it here then the sand can’t fall anymore
            *self.get_mut(point).unwrap() = GridSpace::Sand;
            break true;
        }
    }
}

fn parse_line(line_num: usize,
              point_a: (i32, i32),
              point_b: (i32, i32)) -> Result<Line, String> {
    if point_a.0 == point_b.0 {
        Ok(Line { start: (point_a.0, std::cmp::min(point_a.1, point_b.1)),
                  length: (point_a.1 - point_b.1).abs() + 1,
                  direction: LineDirection::Down })
    } else if point_a.1 == point_b.1 {
        Ok(Line { start: (std::cmp::min(point_a.0, point_b.0), point_a.1),
                  length: (point_a.0 - point_b.0).abs() + 1,
                  direction: LineDirection::Right })
    } else {
        Err(format!("line {}: line is not vertical or horizontal", line_num))
    }
}

fn parse_point(re: &regex::Regex,
               line_num: usize,
               part: &str) -> Result<(i32, i32), String> {
    let captures = match re.captures(part) {
        Some(c) => c,
        None => return Err(format!("line {}: invalid point", line_num)),
    };

    match captures[1].parse::<i32>().and_then(|x| {
        Ok((x, captures[2].parse::<i32>()?))
    }) {
        Err(e) => Err(format!("line {}: {}", line_num, e)),
        Ok(p) => Ok(p),
    }
}

fn read_lines<I>(input_lines: &mut I) -> Result<Vec<Line>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^(\d+),(\d+)$").unwrap();
    let mut lines = Vec::<Line>::new();

    for (line_num, result) in input_lines.enumerate() {
        let input_line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let mut last_point = None;
        let mut had_line = false;

        for part in input_line.split(" -> ") {
            let point = parse_point(&re, line_num + 1, part)?;

            if let Some(last_point) = last_point {
                lines.push(parse_line(line_num + 1, last_point, point)?);
                had_line = true;
            }

            last_point = Some(point);
        }

        if !had_line {
            return Err(format!("line {}: need at least two \
                                points to make a line",
                               line_num + 1));
        }
    }

    if lines.is_empty() {
        return Err("empty input".to_string());
    }

    Ok(lines)
}

fn get_bounds(lines: &[Line]) -> Bounds {
    let mut bounds = Bounds::new();

    for line in lines.iter() {
        bounds.add_point(line.start);
        bounds.add_point(line.end());
    }

    bounds
}

fn fill_grid_with_sand(mut grid: Grid) -> usize {
    for sand_num in 0.. {
        if !grid.add_sand((500, 0)) {
            return sand_num;
        }
    }

    panic!("for loop on infinite range should never end");
}

fn main() -> std::process::ExitCode {
    let mut lines = match read_lines(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(lines) => lines,
    };

    let grid = Grid::new(&lines);
    let bounds = grid.bounds.clone();

    println!("part 1: {}", fill_grid_with_sand(grid));

    // Add a finite line at the bottom of the grid that should have
    // the same effect as an infinite line. It shouldn’t be possible
    // for the sand to go further than 500-height to the left and
    // 500+height to the right.
    let new_y = bounds.max.1 + 2;
    lines.push(Line {
        start: (500 - new_y, new_y),
        length: new_y * 2 + 1,
        direction: LineDirection::Right
    });
    println!("part 2: {}", fill_grid_with_sand(Grid::new(&lines)));

    std::process::ExitCode::SUCCESS
}
