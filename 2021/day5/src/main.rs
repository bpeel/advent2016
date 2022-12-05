use std::collections::HashMap;
use std::cmp::{min,max};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

fn read_lines<I>(text_lines: &mut I) -> Result<Vec<Line>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut lines = Vec::<Line>::new();

    fn parse_num(line_num: usize, s: &str) -> Result<i32, String>
    {
        match s.parse::<i32>() {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("line: {}: {}", line_num + 1, e)),
        }
    }

    for (line_num, result) in text_lines.enumerate() {
        let line = match result {
            Ok(line) => line,
            Err(e) => return Err(e.to_string()),
        };

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line: {}: invalid syntax",
                                       line_num + 1)),
        };

        lines.push(Line {
            start: Point {
                x: parse_num(line_num, &captures[1])?,
                y: parse_num(line_num, &captures[2])?,
            },
            end: Point {
                x: parse_num(line_num, &captures[3])?,
                y: parse_num(line_num, &captures[4])?,
            },
        });
    }

    Ok(lines)
}

fn draw_lines(lines: &[Line]) -> HashMap<(i32, i32), u32> {
    let mut map = HashMap::new();

    for line in lines {
        if line.start.x == line.end.x {
            let start = min(line.start.y, line.end.y);
            let end = max(line.start.y, line.end.y);

            for y in start..=end {
                map.entry((line.start.x, y))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        } else if line.start.y == line.end.y {
            let start = min(line.start.x, line.end.x);
            let end = max(line.start.x, line.end.x);

            for x in start..=end {
                map.entry((x, line.start.y))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    map
}

fn main() {
    let lines = match read_lines(&mut std::io::stdin().lines()) {
        Ok(lines) => lines,

        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    };

    let map = draw_lines(&lines);

    let part1 = map.values().filter(|&&count| count > 1).count();

    println!("part 1: {}", part1);
}
