use std::process::ExitCode;
use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum DanceError {
    SpinOutOfRange(usize),
    ExchangeOutOfRange(usize),
    PartnerNotFound(char),
}

impl fmt::Display for DanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DanceError::SpinOutOfRange(length) => {
                write!(f, "spin out of range: {}", length)
            },
            DanceError::ExchangeOutOfRange(pos) => {
                write!(f, "exchange out of range: {}", pos)
            },
            DanceError::PartnerNotFound(partner) => {
                write!(f, "partner not found: {}", partner)
            },
        }
    }
}

impl FromStr for DanceMove {
    type Err = ();

    fn from_str(s: &str) -> Result<DanceMove, ()> {
        if let Some(tail) = s.strip_prefix("s") {
            Ok(DanceMove::Spin(tail.parse::<usize>().map_err(|_| ())?))
        } else if let Some(tail) = s.strip_prefix("x") {
            let (a, b) = tail.split_once('/').ok_or(())?;
            Ok(DanceMove::Exchange(
                a.parse::<usize>().map_err(|_| ())?,
                b.parse::<usize>().map_err(|_| ())?,
            ))
        } else if let Some(tail) = s.strip_prefix("p") {
            let (a, b) = tail.split_once('/').ok_or(())?;
            Ok(DanceMove::Partner(
                a.parse::<char>().map_err(|_| ())?,
                b.parse::<char>().map_err(|_| ())?,
            ))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for DanceMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DanceMove::Spin(length) => write!(f, "s{}", length),
            DanceMove::Exchange(a, b) => write!(f, "x{}/{}", a, b),
            DanceMove::Partner(a, b) => write!(f, "p{}/{}", a, b),
        }
    }
}

impl DanceMove {
    fn apply(&self, dance_hall: &mut [char]) -> Result<(), DanceError> {
        match self {
            DanceMove::Spin(length) => {
                if *length > dance_hall.len() {
                    return Err(DanceError::SpinOutOfRange(*length));
                }

                dance_hall.rotate_right(*length);

                Ok(())
            },
            DanceMove::Exchange(a, b) => {
                if *a >= dance_hall.len() {
                    return Err(DanceError::ExchangeOutOfRange(*a));
                }
                if *b >= dance_hall.len() {
                    return Err(DanceError::ExchangeOutOfRange(*b));
                }

                dance_hall.swap(*a, *b);

                Ok(())
            },
            DanceMove::Partner(a, b) => {
                let a = dance_hall.iter().position(|&x| x == *a)
                    .ok_or_else(|| DanceError::PartnerNotFound(*a))?;
                let b = dance_hall.iter().position(|&x| x == *b)
                    .ok_or_else(|| DanceError::PartnerNotFound(*b))?;

                dance_hall.swap(a, b);

                Ok(())
            },
        }
    }
}

fn read_dance_moves() -> Result<Vec<DanceMove>, String> {
    std::io::read_to_string(std::io::stdin())
        .map_err(|e| e.to_string())?
        .trim_end()
        .split(',')
        .map(|s| {
            s.parse::<DanceMove>().map_err(|_| {
                format!("invalid dance move: {}", s)
            })
        }).collect()
}

fn make_loop(
    dance_moves: &[DanceMove],
) -> Result<(Vec<String>, usize), DanceError> {
    let mut dance_hall = (0..16).map(|pos| {
        char::from_u32(pos + 'a' as u32).unwrap()
    }).collect::<Vec<_>>();

    let mut loop_list = vec![dance_hall.iter().cloned().collect::<String>()];

    let mut seen = HashMap::from([(loop_list[0].clone(), 0)]);

    loop {
        for dance_move in dance_moves.iter() {
            dance_move.apply(&mut dance_hall)?;
        }

        let order = dance_hall.iter().cloned().collect::<String>();

        if let Some(loop_start) = seen.insert(
            order.clone(),
            loop_list.len()
        ) {
            return Ok((loop_list, loop_start));
        }

        loop_list.push(order);
    }
}

fn main() -> ExitCode {
    let dance_moves = match read_dance_moves() {
        Ok(dm) => dm,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let (dance_loop, loop_start) = match make_loop(&dance_moves) {
        Ok(dl) => dl,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    for (part, iterations) in [1, 1_000_000_000].into_iter().enumerate() {
        let index = if iterations < loop_start {
            iterations
        } else {
            (iterations - loop_start) % dance_loop.len() + loop_start
        };

        println!(
            "Part {}: {}",
            part + 1,
            dance_loop[index],
        );
    }

    ExitCode::SUCCESS
}
