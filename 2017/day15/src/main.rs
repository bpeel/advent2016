static FACTORS: [u64; 2] = [16807, 48271];
static MODULO: u64 = 2147483647;

struct Generator {
    value: u64,
    factor: u64,
    filter: u64,
}

impl Generator {
    fn new(initial_value: u64, factor: u64, filter: u64) -> Generator {
        Generator {
            value: initial_value,
            factor,
            filter,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.value = (self.value * self.factor) % MODULO;

            if self.value & self.filter == 0 {
                break Some(self.value);
            }
        }
    }
}

fn read_initial_values() -> Option<[u64; 2]> {
    let mut values = [0u64; 2];
    let mut args = std::env::args().skip(1);

    for value in values.iter_mut() {
        *value = args.next()?.parse::<u64>().ok()?;
    }

    Some(values)
}

fn inspect<I, J>(
    n_numbers: usize,
    generator_a: I,
    generator_b: J,
) -> u32
    where I: IntoIterator<Item = u64>,
          J: IntoIterator<Item = u64>
{
    generator_a.into_iter().zip(generator_b.into_iter())
        .take(n_numbers)
        .map(|(a, b)| (a & 0xffff == b & 0xffff) as u32)
        .sum::<u32>()
}

fn main() -> std::process::ExitCode {
    let Some(initial_values) = read_initial_values()
    else {
        eprintln!("usage: day15 <a_start> <b_start>");
        return std::process::ExitCode::FAILURE;
    };

    println!(
        "Part 1: {}",
        inspect(
            40_000_000,
            Generator::new(initial_values[0], FACTORS[0], 0),
            Generator::new(initial_values[1], FACTORS[1], 0),
        ),
    );

    println!(
        "Part 2: {}",
        inspect(
            5_000_000,
            Generator::new(initial_values[0], FACTORS[0], 3),
            Generator::new(initial_values[1], FACTORS[1], 7),
        ),
    );

    return std::process::ExitCode::SUCCESS;
}
