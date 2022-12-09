use std::collections::HashSet;

#[derive(Debug, Clone)]
struct RopeLink {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Rope {
    links: Vec<RopeLink>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        Rope { links: vec![RopeLink { x: 0, y: 0 }; size] }
    }

    fn move_head(&mut self, direction_start: (i32, i32)) {
        let mut direction = direction_start;

        for head_pos in 0..self.links.len() {
            let head = &mut self.links[head_pos];

            head.x += direction.0;
            head.y += direction.1;

            if head_pos >= self.links.len() - 1 {
                break;
            }

            let head = &self.links[head_pos];
            let tail = &self.links[head_pos + 1];

            if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
                break;
            }

            direction = ((head.x - tail.x).signum(),
                         (head.y - tail.y).signum());
        }
    }

    fn tail(&self) -> (i32, i32) {
        let tail = self.links.last().unwrap();
        (tail.x, tail.y)
    }
}

#[derive(Debug, Clone)]
struct Command {
    direction: (i32, i32),
    count: usize,
}

fn read_commands<I>(lines: &mut I) -> Result<Vec<Command>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^([UDLR]) (\d+)$").unwrap();
    let mut commands = Vec::<Command>::new();

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

        let direction = match &captures[1] {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("regex returned impossible char"),
        };

        let count = match captures[2].parse::<usize>() {
            Ok(n) => n,
            Err(e) => return Err(format!("line {}: {}",
                                         line_num + 1,
                                         e)),
        };

        commands.push(Command { direction, count });
    }

    Ok(commands)
}

fn run_part(commands: &[Command], rope_length: usize) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut rope = Rope::new(rope_length);

    visited.insert(rope.tail());

    for command in commands {
        for _ in 0..command.count {
            rope.move_head(command.direction);
            visited.insert(rope.tail());
        }
    }

    visited.len()
}

// Unofficial part 3 from Reddit
// https://www.reddit.com/r/adventofcode/comments/zh4hvm/2022_day_9_shortest_rope_with_fixed_tail/
fn run_part3(commands: &[Command]) -> usize {
    let mut rope = Rope::new(1);

    for command in commands {
        for _ in 0..command.count {
            rope.move_head(command.direction);

            // If the tail has moved away from the center then make
            // the rope longer
            let last = rope.links.last().unwrap();

            if last.x != 0 || last.y != 0 {
                rope.links.push(RopeLink { x: 0, y: 0 });
            }
        }
    }

    rope.links.len()
}

fn main() -> std::process::ExitCode {
    let commands = match read_commands(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(commands) => commands,
    };

    println!("part 1: {}", run_part(&commands, 2));
    println!("part 2: {}", run_part(&commands, 10));
    println!("part 3: {}", run_part3(&commands));

    std::process::ExitCode::SUCCESS
}
