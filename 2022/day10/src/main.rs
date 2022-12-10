const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

#[derive(Debug, Clone)]
enum Instruction {
    Add(i32),
    Noop,
}

fn read_instructions<I>(lines: &mut I) -> Result<Vec<Instruction>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new(r"^addx (-?\d+)$").unwrap();
    let mut instructions = Vec::<Instruction>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        if line == "noop" {
            instructions.push(Instruction::Noop);
            continue;
        }

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line: {}: invalid syntax",
                                       line_num + 1)),
        };

        let operand = match captures[1].parse::<i32>() {
            Ok(o) => o,
            Err(e) => return Err(format!("line {}: {}",
                                         line_num + 1,
                                         e.to_string())),
        };

        instructions.push(Instruction::Add(operand));
    }

    Ok(instructions)
}

fn main() -> std::process::ExitCode {
    let instructions = match read_instructions(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(instructions) => instructions,
    };

    let mut ireg = 1i32;
    let mut clock = 0;
    let mut screen = vec![false; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut part1 = 0;
    let mut next_target_cycle = 20;

    'instruction_loop: for instruction in instructions {
        let count = match instruction {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        };

        // part 1
        if clock + count >= next_target_cycle {
            part1 += next_target_cycle as i32 * ireg;
            next_target_cycle += 40;
        }

        // part 2
        for i in 0..count {
            let y = (clock + i) / SCREEN_WIDTH;

            if y >= SCREEN_HEIGHT {
                break 'instruction_loop;
            }

            let x = (clock + i) % SCREEN_WIDTH;

            screen[y * SCREEN_WIDTH + x] = (x as i32 - ireg).abs() <= 1;
        }

        if let Instruction::Add(x) = instruction {
            ireg += x;
        }

        clock += count;
    }

    println!("part 1: {}", part1);

    println!("part 2:");

    for y in 0..SCREEN_HEIGHT {
        let line_bools = &screen[y * SCREEN_WIDTH..(y + 1) * SCREEN_WIDTH];
        let line = line_bools.iter()
            .map(|&v| if v { '█' } else { '·' })
            .collect::<String>();

        println!("{}", line);
    }

    std::process::ExitCode::SUCCESS
}
