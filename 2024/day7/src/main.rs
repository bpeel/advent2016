use std::process::ExitCode;
use std::str::FromStr;

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Equation, String> {
        let Some((test_value_str, numbers_str)) = s.split_once(": ")
        else {
            return Err("missing colon".to_string());
        };

        let Ok(test_value) = test_value_str.parse::<u64>()
        else {
            return Err("bad test value".to_string());
        };

        let mut numbers = Vec::new();

        for number in numbers_str.split_whitespace() {
            let Ok(number) = number.parse::<u64>()
            else {
                return Err(format!("bad number: {}", number));
            };

            numbers.push(number);
        }

        if numbers.len() < 2 {
            return Err("not enough numbers".to_string());
        }

        if numbers.len() > 31 {
            return Err("too many numbers".to_string());
        }

        Ok(Equation {
            test_value,
            numbers,
        })
    }
}

fn read_equations<I>(lines: I) -> Result<Vec<Equation>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut equations = Vec::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        match line.parse::<Equation>() {
            Ok(e) => equations.push(e),
            Err(e) => return Err(format!("line {}: {}", line_num + 1, e)),
        }
    }

    Ok(equations)
}

fn find_solution(equation: &Equation) -> Option<u32> {
    let n_combinations = 1u32 << (equation.numbers.len() - 1);

    for chosen_operators in 0..n_combinations {
        let mut result = equation.numbers[0];

        for (i, &number) in equation.numbers[1..].iter().enumerate() {
            if chosen_operators & (1u32 << i) == 0 {
                result += number;
            } else {
                result *= number;
            }
        }

        if result == equation.test_value {
            return Some(chosen_operators);
        }
    }

    None
}

fn part1(equations: &[Equation]) -> u64 {
    equations.iter().filter_map(|equation| {
        find_solution(equation).is_some().then_some(equation.test_value)
    }).sum::<u64>()
}

fn elephant_operator(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a * 10;
    }

    let b_length = b.ilog10() + 1;

    a * 10u64.pow(b_length) + b
}

fn find_elephant_solution(equation: &Equation) -> Option<u32> {
    let n_combinations = 3u32.pow(equation.numbers.len() as u32 - 1);

    for mut chosen_operators in 0..n_combinations {
        let mut result = equation.numbers[0];

        for &number in equation.numbers[1..].iter() {
            match chosen_operators % 3 {
                0 => result += number,
                1 => result *= number,
                2 => result = elephant_operator(result, number),
                _ => unreachable!("mod 3 is broken"),
            }

            chosen_operators /= 3;
        }

        if result == equation.test_value {
            return Some(chosen_operators);
        }
    }

    None
}

fn part2(equations: &[Equation]) -> u64 {
    equations.iter().filter_map(|equation| {
        find_elephant_solution(equation)
            .is_some()
            .then_some(equation.test_value)
    }).sum::<u64>()
}

fn main() -> ExitCode {
    let equations = match read_equations(std::io::stdin().lines()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    println!("Part 1: {}", part1(&equations));
    println!("Part 2: {}", part2(&equations));

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elephant() {
        assert_eq!(elephant_operator(10, 0), 100);
        assert_eq!(elephant_operator(1, 2), 12);
        assert_eq!(elephant_operator(999, 9), 9999);
        assert_eq!(elephant_operator(123456789, 56), 12345678956);
    }
}
