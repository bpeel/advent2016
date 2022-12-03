fn parse_compartment(desc: &[u8]) -> u64 {
    let mut mask: u64 = 0;

    for letter in desc {
        // Can’t use “as” in a match?
        let bit_num = if (('a' as u8)..=('z' as u8)).contains(letter) {
            letter - 'a' as u8
        } else if (('A' as u8)..=('Z' as u8)).contains(letter) {
            letter - 'A' as u8 + 26
        } else {
            continue;
        };

        mask |= (1 as u64) << bit_num;
    };

    mask
}

fn parse_rucksack(line: &str) -> (u64, u64) {
    let bytes = line.as_bytes();
    let mid = bytes.len() / 2;

    (parse_compartment(&bytes[0..mid]), parse_compartment(&bytes[mid..]))
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|result| {
        match result {
            Err(..) => {
                eprintln!("Error reading stdin");
                std::process::exit(1);
            },
            Ok(line) => line.trim_end().to_string(),
        }
    }).collect();

    let part1: u32 = lines.iter().map(|line| {
        let (a, b) = parse_rucksack(line);
        match (a & b).trailing_zeros() {
            64 => 0,
            zeros => zeros + 1,
        }
    }).sum();

    println!("part1: {}", part1);
}
