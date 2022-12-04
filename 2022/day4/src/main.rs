type SectionRange = std::ops::RangeInclusive<u32>;

#[derive(Clone, Debug)]
struct ElfPair {
    a: SectionRange,
    b: SectionRange,
}

fn range_contains(a: &SectionRange, b: &SectionRange) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

impl ElfPair {
    fn is_redundant(&self) -> bool {
        range_contains(&self.a, &self.b) ||
            range_contains(&self.b, &self.a)
    }

    fn overlaps(&self) -> bool {
        self.a.end() >= self.b.start() &&
            self.a.start() <= self.b.end()
    }
}

fn parse_range(s: &str) -> Result<SectionRange, String> {
    let hyphen = match s.find('-') {
        Some(n) => n,
        None => return Err("Range is missing a hyphen".to_string()),
    };

    let a = match s[0..hyphen].parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };

    let b = match s[hyphen + 1..].parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };

    if a > b {
        return Err("Start of range is greater than end".to_string());
    }

    Ok(a..=b)
}

impl std::str::FromStr for ElfPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comma = match s.find(',') {
            Some(n) => n,
            None => return Err("Elf pair string has no comma".to_string()),
        };

        let a = match parse_range(&s[0..comma]) {
            Err(e) => return Err(e),
            Ok(r) => r,
        };

        let b = match parse_range(&s[comma + 1..]) {
            Err(e) => return Err(e),
            Ok(r) => r,
        };

        Ok(ElfPair { a, b })
    }
}

fn main() {
    let pairs: Vec<ElfPair> = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(line_num, result)| {
            match result {
                Err(..) => {
                    eprintln!("I/O error");
                    std::process::exit(1);
                },
                Ok(line) => match line.parse::<ElfPair>() {
                    Err(e) => {
                        eprintln!("{}: {}", line_num + 1, e);
                        std::process::exit(1);
                    },
                    Ok(pair) => pair,
                }
            }
        })
        .collect();

    let part1 = pairs.iter().filter(|pair| pair.is_redundant()).count();

    println!("part 1: {}", part1);

    let part2 = pairs.iter().filter(|pair| pair.overlaps()).count();

    println!("part 2: {}", part2);
}
