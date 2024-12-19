fn read_initial_values() -> Option<[u64; 2]> {
    let mut values = [0u64; 2];
    let mut args = std::env::args().skip(1);

    for value in values.iter_mut() {
        *value = args.next()?.parse::<u64>().ok()?;
    }

    Some(values)
}

fn main() -> std::process::ExitCode {
    let Some(mut values) = read_initial_values()
    else {
        eprintln!("usage: day15 <a_start> <b_start>");
        return std::process::ExitCode::FAILURE;
    };

    let part1 = (0..40_000_000).map(|_| {
        for (i, factor) in [16807, 48271].into_iter().enumerate() {
            values[i] = (values[i] * factor) % 2147483647;
        }

        (values[0] & 0xffff == values[1] & 0xffff) as u32
    }).sum::<u32>();

    println!("Part 1: {}", part1);

    return std::process::ExitCode::SUCCESS;
}
