use std::process::ExitCode;
use std::ffi::OsString;
use std::fmt;

struct Stones {
    stones: Vec<u32>,
    temp_buf: Vec<u32>,
}

impl Stones {
    fn new(stones: Vec<u32>) -> Stones {
        Stones {
            stones,
            temp_buf: Vec::new(),
        }
    }

    fn step(&mut self) {
        self.temp_buf.clear();

        for &stone in self.stones.iter() {
            if stone == 0 {
                self.temp_buf.push(1);
            } else {
                let n_digits = stone.ilog10() + 1;

                if n_digits & 1 == 0 {
                    let divisor = 10u32.pow(n_digits / 2);
                    self.temp_buf.push(stone / divisor);
                    self.temp_buf.push(stone % divisor);
                } else {
                    self.temp_buf.push(stone * 2024);
                }
            }
        }

        std::mem::swap(&mut self.temp_buf, &mut self.stones);
    }
}

impl fmt::Display for Stones {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, &stone) in self.stones.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }

            write!(f, "{}", stone)?;
        }

        Ok(())
    }
}

fn parse_stones<'a, I>(
    args: I,
) -> Result<Vec<u32>, String>
    where I: IntoIterator<Item = OsString>
{
    let mut stones = Vec::new();

    for arg in args {
        let Some(arg_str) = arg.to_str()
        else {
            return Err(format!("bad number: {}", arg.to_string_lossy()));
        };

        match arg_str.parse::<u32>() {
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

    for _ in 0..25 {
        stones.step();
    }

    println!("Part 1: {}", stones.stones.len());

    ExitCode::SUCCESS
}
