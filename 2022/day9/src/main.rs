use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Item {
    direction: char,
    count: usize,
}

fn read_items<I>(lines: &mut I) -> Result<Vec<Item>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^([UDLR]) (\d+)$").unwrap();
    let mut items = Vec::<Item>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => e.to_string(),
            Ok(line) => line,
        };

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line: {}: invalid syntax",
                                       line_num + 1)),
        };

        items.push(Item {
            direction: (&captures[1]).chars().nth(0).unwrap(),
            count: captures[2].parse::<usize>().unwrap(),
        });
    }

    Ok(items)
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn move_rope(rope: &mut [Pos], direction: (i32, i32)) {
    let head = rope.len() - 1;

    rope[head].x += direction.0;
    rope[head].y += direction.1;

    if head == 0 {
        return;
    }

    let tail = head - 1;

    let horiz = (rope[head].x - rope[tail].x).abs() > 1;
    let vert = (rope[head].y - rope[tail].y).abs() > 1;

    if horiz || vert {
        let dir = ((rope[head].x - rope[tail].x).signum(),
                   (rope[head].y - rope[tail].y).signum());
        move_rope(&mut rope[0..head], dir);
    }
}

fn main() -> std::process::ExitCode {
    let items;

    {
        let input = std::io::stdin().lock();

        items = match read_items(&mut input.lines()) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(items) => items,
        };
    }

    let mut visited = HashSet::<Pos>::new();
    let rope_start = Pos { x: 0, y: 0 };
    let mut rope: Vec<Pos> = std::iter::repeat(rope_start).take(10).collect();

    visited.insert(rope[0].clone());

    for item in items {
        for _ in 0..item.count {
            let direction = match item.direction {
                'U' => {
                    (0, -1)
                },
                'D' => {
                    (0, 1)
                },
                'L' => {
                    (-1, 0)
                },
                'R' => {
                    (1, 0)
                },
                _ => panic!("??"),
            };
            move_rope(&mut rope, direction);
            println!("{:?} {:?} {:?}", item.direction, rope[9], rope[0]);
            visited.insert(rope[0].clone());
        }
    }

    println!("{:?}", visited.len());

    std::process::ExitCode::SUCCESS
}
