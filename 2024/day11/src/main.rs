use std::process::ExitCode;
use std::ffi::OsString;
use std::collections::HashMap;

struct Stones {
    stones: HashMap<u64, u64>,
    temp_buf: HashMap<u64, u64>,
}

fn add_stones(stones: &mut HashMap<u64, u64>, stone: u64, count: u64) {
    stones.entry(stone)
        .and_modify(|old_count| *old_count += count)
        .or_insert(count);
}

impl Stones {
    fn new<I: IntoIterator<Item = u64>>(stone_numbers: I) -> Stones {
        let mut stones = HashMap::new();

        for stone in stone_numbers {
            add_stones(&mut stones, stone, 1);
        }

        Stones {
            stones,
            temp_buf: HashMap::new(),
        }
    }

    fn step(&mut self) {
        self.temp_buf.clear();

        for (&stone, &count) in self.stones.iter() {
            if stone == 0 {
                add_stones(&mut self.temp_buf, 1, count);
            } else {
                let n_digits = stone.ilog10() + 1;

                if n_digits & 1 == 0 {
                    let divisor = 10u64.pow(n_digits / 2);
                    add_stones(&mut self.temp_buf, stone / divisor, count);
                    add_stones(&mut self.temp_buf, stone % divisor, count);
                } else {
                    add_stones(&mut self.temp_buf, stone * 2024, count);
                }
            }
        }

        std::mem::swap(&mut self.temp_buf, &mut self.stones);
    }

    fn len(&self) -> u64 {
        self.stones.values().sum::<u64>()
    }
}

fn parse_stones<'a, I>(
    args: I,
) -> Result<Vec<u64>, String>
    where I: IntoIterator<Item = OsString>
{
    let mut stones = Vec::new();

    for arg in args {
        let Some(arg_str) = arg.to_str()
        else {
            return Err(format!("bad number: {}", arg.to_string_lossy()));
        };

        match arg_str.parse::<u64>() {
            Ok(number) => stones.push(number),
            Err(_) => return Err(format!("bad number: {}", arg_str)),
        };
    }

    Ok(stones)
}

fn main() -> ExitCode {
    let mut stones = match parse_stones(std::env::args_os().skip(1)) {
        Ok(s) => Stones::new(s),
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    for i in 0..75 {
        if i == 25 {
            println!("Part 1: {}", stones.len());
        }

        stones.step();
    }

    println!("Part 2: {}", stones.len());

    ExitCode::SUCCESS
}
