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

fn main() -> ExitCode {
    for arg in std::env::args_os().skip(1) {
        let count = (0..128).map(|row| {
            let suffix = format!("-{}", row);
            let data = arg.as_encoded_bytes().into_iter().cloned()
                .chain(suffix.as_bytes().into_iter().cloned());
            hash(data).count_ones()
        }).sum::<u32>();

        println!("Part 1: {}", count);
    }

    ExitCode::SUCCESS
}
