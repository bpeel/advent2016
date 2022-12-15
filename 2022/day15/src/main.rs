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
    // Temporary interval set used to avoid reallocating every time we
    // subtract a range
    temp_intervals: Vec<Interval>,
}

impl CoordSet {
    fn new() -> CoordSet {
        CoordSet {
            intervals: vec![Interval::new(i32::MIN, i32::MAX)],
            temp_intervals: Vec::<Interval>::new(),
        }
    }

    fn subtract(&mut self, start: i32, end: i32) {
        self.temp_intervals.clear();

        for interval in self.intervals.iter() {
            // If the interval doesn’t overlap the range to subtract
            // then leave it alone
            if end <= interval.start || start >= interval.end {
                self.temp_intervals.push(*interval);
                continue;
            }

            if start > interval.start {
                self.temp_intervals.push(Interval::new(interval.start, start));
            }

            if end < interval.end {
                self.temp_intervals.push(Interval::new(end, interval.end));
            }
        }

        std::mem::swap(&mut self.intervals, &mut self.temp_intervals);
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

fn apply_sensors(set: &mut CoordSet,
                 test_row: i32,
                 sensors: &[Sensor],
                 exclude_beacon_pos: bool) {
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

        if exclude_beacon_pos {
            // Special case if the beacon is on the test row. In that
            // case we want to exclude the beacon position from the
            // range because there definitely is a beacon there
            if sensor.beacon_pos.1 == test_row {
                if sensor.beacon_pos.0 < sensor.sensor_pos.0 {
                    range_start += 1;
                } else {
                    range_end -= 1;
                }
            }
        }

        set.subtract(range_start, range_end);
    }
}

fn run_part1(test_row: i32, sensors: &[Sensor]) -> usize {
    let mut set = CoordSet::new();

    apply_sensors(&mut set, test_row, sensors, true);

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


fn run_part2(test_range: i32, sensors: &[Sensor]) -> String {
    let mut found_pos = None;

    for test_row in 0..test_range {
        let mut set = CoordSet::new();

        set.subtract(i32::MIN, 0);
        set.subtract(test_range, i32::MAX);

        apply_sensors(&mut set, test_row, sensors, false);

        for interval in set.intervals.iter() {
            for x in interval.start..interval.end {
                if let Some(old_pos) = found_pos {
                    return format!("Possible beacon at {:?} but there is \
                                    already a possible beacon at {:?}",
                                   (x, test_row),
                                   old_pos);
                }

                found_pos = Some((x, test_row));
            }
        }
    }

    match found_pos {
        Some((x, y)) => format!("{}", x as i64 * 4_000_000 + y as i64),
        None => "no possible beacon position found".to_string(),
    }
}

fn parse_int_arg<T>(s: Option<std::ffi::OsString>, default: T)
                    -> Result<T, String>
    where T: std::str::FromStr
{
    match s {
        None => Ok(default),
        Some(os_string) => match os_string.into_string() {
            Err(_) => Err("invalid UTF-8 string in argument".to_string()),
            Ok(s) => match s.parse::<T>() {
                Ok(n) => Ok(n),
                Err(_) => Err("invalid integer arg".to_string()),
            },
        },
    }
}

fn main() -> std::process::ExitCode {
    let sensors = match read_sensors(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(sensors) => sensors,
    };

    let mut args = std::env::args_os();

    let (test_row, test_range) = match parse_int_arg(args.nth(1), 2_000_000)
        .and_then(|row| Ok((row, parse_int_arg(args.next(), 4_000_000)?))) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(v) => v,
        };

    println!("part 1: {}", run_part1(test_row, &sensors));
    println!("part 2: {}", run_part2(test_range, &sensors));

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
