fn parse_snafu(s: &str) -> Result<i32, String> {
    let mut digit_value = 1;
    let mut total = 0;

    for ch in s.chars().rev() {
        let mult = match ch {
            '0'..='2' => ch as i32 - '0' as i32,
            '-' => -1,
            '=' => -2,
            _ => return Err("invalid char in snafu".to_string()),
        };

        total += mult * digit_value;
        digit_value *= 5;
    }

    Ok(total)
}

fn to_snafu(mut value: i32) -> String {
    let mut carry = 0;
    let mut digits = String::new();

    while value > 0 || carry > 0 {
        let digit = (value % 5) + carry;

        if digit <= 2 {
            digits.push(char::from_digit(digit as u32, 10).unwrap());
            carry = 0;
        } else {
            let digit = match digit {
                3 => '=',
                4 => '-',
                5 => '0',
                _ => panic!("impossible digit!"),
            };
            digits.push(digit);
            carry = 1;
        }

        value /= 5;
    }

    digits.chars().rev().collect::<String>()
}

fn main() -> std::process::ExitCode {
    let mut total = Some(0);

    for result in std::io::stdin().lines() {
        let line = match result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
        };

        print!("{:20}: ", line);

        match parse_snafu(&line) {
            Ok(v) => {
                println!("{} ({})", v, to_snafu(v));
                if let Some(s) = total {
                    total = Some(s + v);
                }
            },
            Err(e) => {
                println!("{}", e);
                total = None;
            },
        }
    }
    
    if let Some(total) = total {
        println!("total = {}", total);
        println!("part 1 = {}", to_snafu(total));
    }

    std::process::ExitCode::SUCCESS
}
