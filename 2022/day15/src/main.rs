#[derive(Debug, Clone)]
struct Sensor {
    sensor_pos: (i32, i32),
    beacon_pos: (i32, i32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Interval {
        assert!(start < end);
        Interval { start, end }
    }
}

#[derive(Debug, Clone)]
struct CoordSet {
    intervals: Vec<Interval>,
}

impl CoordSet {
    fn new() -> CoordSet {
        CoordSet { intervals: vec![Interval::new(i32::MIN, i32::MAX)] }
    }

    fn subtract(&mut self, start: i32, end: i32) {
        let mut new_points = Vec::<Interval>::new();

        for interval in self.intervals.iter() {
            // If the interval doesn’t overlap the range to subtract
            // then leave it alone
            if end <= interval.start || start >= interval.end {
                new_points.push(*interval);
                continue;
            }

            if start > interval.start {
                new_points.push(Interval::new(interval.start, start));
            }

            if end < interval.end {
                new_points.push(Interval::new(end, interval.end));
            }
        }

        self.intervals = new_points;
    }
}

impl std::fmt::Display for CoordSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
           -> Result<(), std::fmt::Error> {
        for (i, interval) in self.intervals.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "({}..{})", interval.start, interval.end)?;
        }

        Ok(())
    }
}

fn read_sensors<I>(lines: &mut I) -> Result<Vec<Sensor>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new("Sensor at x=(-?\\d+), y=(-?\\d+): \
                                closest beacon is at \
                                x=(-?\\d+), y=(-?\\d+)$").unwrap();
    let mut sensors = Vec::<Sensor>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line: {}: invalid syntax",
                                       line_num + 1)),
        };

        sensors.push(match captures[1].parse::<i32>()
                     .and_then(|x| Ok((x, captures[2].parse::<i32>()?)))
                     .and_then(|pos| Ok((pos, captures[3].parse::<i32>()?)))
                     .and_then(|(sensor_pos, x)| Ok(Sensor {
                         sensor_pos,
                         beacon_pos: (x, captures[4].parse::<i32>()?)
                     })) {
                         Err(e) => return Err(format!("line: {}: {}",
                                                      line_num + 1,
                                                      e)),
                         Ok(s) => s,
                     });
    }

    if sensors.len() == 0 {
        return Err("empty sensors list".to_string());
    }

    Ok(sensors)
}

fn run_part1(test_row: i32, sensors: &[Sensor]) -> usize {
    let mut set = CoordSet::new();

    for sensor in sensors.iter() {
        let distance_to_beacon =
            (sensor.sensor_pos.0 - sensor.beacon_pos.0).abs() +
            (sensor.sensor_pos.1 - sensor.beacon_pos.1).abs();

        let y_distance = (test_row - sensor.sensor_pos.1).abs();
        let row_distance = distance_to_beacon - y_distance;

        // If the range we want to subtract doesn’t reach the test row
        // then skip it
        if row_distance < 0 {
            continue;
        }

        let mut range_start = sensor.sensor_pos.0 - row_distance;
        let mut range_end = sensor.sensor_pos.0 + row_distance + 1;

        // Special case if the beacon is on the test row. In that case
        // we want to exclude the beacon position from the range
        // because there definitely is a beacon there
        if sensor.beacon_pos.1 == test_row {
            if sensor.beacon_pos.0 < sensor.sensor_pos.0 {
                range_start += 1;
            } else {
                range_end -= 1;
            }
        }

        set.subtract(range_start, range_end);
    }

    let mut total_spaces = 0;

    if set.intervals.len() >= 2 {
        for i in 0..set.intervals.len() - 1 {
            let left = set.intervals[i];
            let right = set.intervals[i + 1];

            total_spaces += (right.start - left.end) as usize;
        }
    }

    total_spaces
}

fn main() -> std::process::ExitCode {
    let sensors = match read_sensors(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(sensors) => sensors,
    };

    let test_row = match std::env::args_os().nth(1) {
        None => 2_000_000,
        Some(os_string) => match os_string.into_string() {
            Err(_) => {
                eprintln!("invalid UTF-8 string in argument");
                return std::process::ExitCode::FAILURE;
            },
            Ok(s) => match s.parse::<i32>() {
                Ok(row) => row,
                Err(e) => {
                    eprintln!("invalid test row: {}", e);
                    return std::process::ExitCode::FAILURE;
                },
            },
        },
    };

    println!("part 1: {}", run_part1(test_row, &sensors));

    std::process::ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coord_set() {
        // Cut the right side off
        let mut set = CoordSet::new();
        set.subtract(15, i32::MAX);
        assert_eq!(set.to_string(), "(-2147483648..15)");

        // Cut the left side off
        let mut set = CoordSet::new();
        set.subtract(i32::MIN, 12);
        assert_eq!(set.to_string(), "(12..2147483647)");

        // Cut the middle out
        let mut set = CoordSet::new();
        set.subtract(12, 15);
        assert_eq!(set.to_string(), "(-2147483648..12), (15..2147483647)");

        // Cut the middle out again
        set.subtract(17, 18);
        assert_eq!(set.to_string(),
                   "(-2147483648..12), (15..17), (18..2147483647)");

        // Consume the middle range
        set.subtract(15, 17);
        assert_eq!(set.to_string(), "(-2147483648..12), (18..2147483647)");

        // Subtract outside of the set
        set.subtract(12, 18);
        assert_eq!(set.to_string(), "(-2147483648..12), (18..2147483647)");
    }
}
