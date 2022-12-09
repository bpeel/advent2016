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

#[derive(Clone, Debug)]
struct Rope {
    head: Pos,
    tail: Pos,
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
    let mut rope = Rope { head: Pos { x: 0, y: 0 }, tail: Pos { x: 0, y: 0 } };

    visited.insert(rope.tail.clone());

    for item in items {
        for _ in 0..item.count {
            match item.direction {
                'U' => {
                    rope.head.y -= 1;
                },
                'D' => {
                    rope.head.y += 1;
                },
                'L' => {
                    rope.head.x -= 1;
                },
                'R' => {
                    rope.head.x += 1;
                },
                _ => panic!("??"),
            }
            let horiz = (rope.head.x - rope.tail.x).abs() > 1;
            let vert = (rope.head.y - rope.tail.y).abs() > 1;
            if horiz {
                rope.tail.x += (rope.head.x - rope.tail.x).signum();
                if rope.tail.y != rope.head.y {
                    rope.tail.y += (rope.head.y - rope.tail.y).signum();
                }
            } else if vert {
                rope.tail.y += (rope.head.y - rope.tail.y).signum();
                if rope.tail.x != rope.head.x {
                    rope.tail.x += (rope.head.x - rope.tail.x).signum();
                }
            }
            println!("{:?} {:?} {:?}", item.direction, rope.head, rope.tail);
            visited.insert(rope.tail.clone());
        }
    }

    println!("{:?}", visited.len());

    std::process::ExitCode::SUCCESS
}
