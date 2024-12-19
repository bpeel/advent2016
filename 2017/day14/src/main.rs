use std::process::ExitCode;

struct Hasher {
    string: [u8; 256],
    current_pos: usize,
    skip_size: usize,
}

impl Hasher {
    fn new() -> Hasher {
        let mut string = [0u8; 256];

        for (i, number) in string.iter_mut().enumerate() {
            *number = i as u8;
        }

        Hasher {
            string,
            current_pos: 0,
            skip_size: 0,
        }
    }

    fn add_length(&mut self, length: u8) {
        for i in 0..length / 2 {
            self.string.swap(
                (self.current_pos + i as usize) & 0xff,
                (self.current_pos + (length - i) as usize - 1) & 0xff,
            )
        }

        self.current_pos += (self.skip_size + length as usize) & 0xff;
        self.skip_size = (self.skip_size + 1) & 0xff;
    }

    fn hash(&self) -> u128 {
        let mut result = 0;

        for i in 0..16 {
            let part = self.string[i * 16..(i + 1) * 16]
                .iter()
                .fold(0u8, |a, b| a ^ b);
            result = (result << 8) | (part as u128);
        }

        result
    }
}

fn hash<I>(
    values: I,
) -> u128
    where I: IntoIterator<Item = u8, IntoIter: Clone>
{
    let iterator = values.into_iter();
    let mut hasher = Hasher::new();

    for _ in 0..64 {
        for byte in iterator.clone() {
            hasher.add_length(byte);
        }

        for byte in [17, 31, 73, 47, 23] {
            hasher.add_length(byte);
        }
    }

    hasher.hash()
}

fn visit_region(disk: &[u128], visited: &mut [u128], start_pos: usize) {
    let mut stack = vec![start_pos];

    while let Some(pos) = stack.pop() {
        let row = pos / 128;
        let col = pos % 128;
        let bit = 1u128 << col;

        if disk[row] & bit != 0 && visited[row] & bit == 0 {
            visited[row] |= bit;

            if row >= 1 {
                stack.push(pos - 128);
            }
            if row + 1 < 128 {
                stack.push(pos + 128);
            }
            if col >= 1 {
                stack.push(pos - 1);
            }
            if col + 1 < 128 {
                stack.push(pos + 1);
            }
        }
    }
}

fn count_regions(disk: &[u128]) -> u32 {
    let mut visited = [0u128; 128];
    let mut count = 0;

    for (row, &(mut bits)) in disk.iter().enumerate() {
        while bits != 0 {
            let col = bits.trailing_zeros();
            let bit = 1u128 << col;

            if visited[row] & bit == 0 {
                visit_region(disk, &mut visited, row * 128 + col as usize);
                count += 1;
            }

            assert!(visited[row] & bit != 0);

            bits &= !bit;
        }
    }

    count
}

fn main() -> ExitCode {
    let Some(key) = std::env::args_os().nth(1)
    else {
        eprintln!("Usage: day14 <input>");
        return ExitCode::FAILURE;
    };

    let disk = (0..128).map(|row| {
        let suffix = format!("-{}", row);
        let data = key.as_encoded_bytes().into_iter().cloned()
            .chain(suffix.as_bytes().into_iter().cloned());
        hash(data)
    }).collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        disk.iter().map(|row| row.count_ones()).sum::<u32>(),
    );

    println!("Part 2: {}", count_regions(&disk));

    ExitCode::SUCCESS
}
