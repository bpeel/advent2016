use std::collections::HashMap;

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

fn draw_lines(lines: &[Line], draw_diagonals: bool) ->
    HashMap<(i32, i32), u32>
{
    let mut map = HashMap::new();

    for line in lines {
        let x_step = (line.end.x - line.start.x).signum();
        let y_step = (line.end.y - line.start.y).signum();
        let count = if x_step == 0 {
            (line.end.y - line.start.y).abs() + 1
        } else if !draw_diagonals && y_step != 0 {
            continue;
        } else {
            (line.end.x - line.start.x).abs() + 1
        };

        for i in 0..count {
            let index = (line.start.x + i * x_step,
                         line.start.y + i * y_step);
            map.entry(index).and_modify(|count| *count += 1).or_insert(1);
        }
    }

    map
}

fn run_part(lines: &[Line], draw_diagonals: bool) -> usize {
    let map = draw_lines(&lines, draw_diagonals);

    map.values().filter(|&&count| count > 1).count()
}

fn main() {
    let lines = match read_lines(&mut std::io::stdin().lines()) {
        Ok(lines) => lines,

        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    };

    println!("part 1: {}", run_part(&lines, false));
    println!("part 2: {}", run_part(&lines, true));
}
