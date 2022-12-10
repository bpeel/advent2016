use std::io::BufRead;

#[derive(Debug, Clone)]
enum Item {
    Add(i32),
    Noop,
}

fn read_items<I>(lines: &mut I) -> Result<Vec<Item>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^addx (-?\d+)$").unwrap();
    let mut items = Vec::<Item>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        if line == "noop" {
            items.push(Item::Noop);
            continue;
        }

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line: {}: invalid syntax",
                                       line_num + 1)),
        };

        items.push(Item::Add(captures[1].parse::<i32>().unwrap()));
    }

    Ok(items)
}

fn main() -> std::process::ExitCode {
    let items;

    {
        let mut input = std::io::stdin().lock();

        items = match read_items(&mut input.lines()) {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(items) => items,
        };
    }

    let mut ireg = 1i32;
    let mut clock = 0;
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut screen = vec![false; WIDTH * HEIGHT];
    let mut scan_pos = 0;

    for item in items {
        let count = match item {
            Item::Noop => 1,
            Item::Add(_) => 2,
        };

        for i in 0..count {
            let x = (clock + i) as usize % WIDTH;
            let y = (clock + i) as usize / WIDTH;
            screen[y * WIDTH + x] = (x as i32 - ireg).abs() <= 1;
        }

        if let Item::Add(x) = item {
            ireg += x;
        }
        clock += count;
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if screen[y * WIDTH + x] { "#" } else { "." });
        }
        println!("");
    }

    std::process::ExitCode::SUCCESS
}
