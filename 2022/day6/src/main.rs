#[derive(Debug, Clone)]
struct ByteSet {
    bits: [u32; ByteSet::N_INTS],
}

impl ByteSet {
    const N_INTS: usize = (u8::MAX as usize + 1) / u32::BITS as usize;

    fn new() -> ByteSet {
        ByteSet { bits: [0; ByteSet::N_INTS] }
    }

    fn get_index(byte: u8) -> (usize, u32) {
        ((byte as u32 / u32::BITS) as usize,
         1u32 << (byte as u32 % u32::BITS))
    }

    fn insert(&mut self, byte: u8) {
        let (byte, mask) = ByteSet::get_index(byte);

        self.bits[byte] |= mask;
    }

    fn remove(&mut self, byte: u8) {
        let (byte, mask) = ByteSet::get_index(byte);

        self.bits[byte] &= !mask;
    }

    fn contains(&self, byte: u8) -> bool {
        let (byte, mask) = ByteSet::get_index(byte);

        self.bits[byte] & mask != 0
    }
}

fn find_marker(data: &str, marker_length: usize) -> Option<usize> {
    let bytes = data.as_bytes();
    let mut set = ByteSet::new();
    let mut start = 0;

    // I changed this after I posted it to Reddit and Mastodon in
    // order to copy the idea from /u/FattThor of keeping track of the
    // starting point of a sequence of characters without repeats.
    // Therefore the set represents the characters that are in this
    // working sequence. Whenever we encounter a character that is
    // already in the set then we remove characters until that
    // duplicate is removed and we advance the start point. We know
    // that the start point of the marker will have to be after that
    // duplicate letter.
    //
    // https://www.reddit.com/r/adventofcode/comments/zdw0u6/comment/iz44uix/

    for (i, &byte) in bytes.iter().enumerate() {
        while set.contains(byte) {
            set.remove(bytes[start]);
            start += 1;
        }

        set.insert(byte);

        if i - start + 1 >= marker_length {
            return Some(i + 1);
        }
    }

    None
}

fn main() {
    for result in std::io::stdin().lines() {
        let line = match result {
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
            Ok(line) => line,
        };

        for (part_num, &marker_length) in [4, 14].iter().enumerate() {
            let result = match find_marker(&line, marker_length) {
                Some(n) => n.to_string(),
                None => "marker not found".to_string(),
            };

            println!("part {}: {}", part_num + 1, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
                   Some(7));
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
                   Some(5));
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
                   Some(10));
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
                   Some(11));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
                   Some(19));
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
                   Some(23));
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
                   Some(29));
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
                   Some(26));
    }
    #[test]
    fn test_extremes() {
        // Beginning of string
        assert_eq!(find_marker("abcde", 4), Some(4));
        // End of string
        assert_eq!(find_marker("aabcd", 4), Some(5));
        // Not found at all
        assert_eq!(find_marker("aabbccddeeffgghhiijj", 4), None);
        // String too short
        assert_eq!(find_marker("abc", 4), None);
        // Entire string
        assert_eq!(find_marker("abcd", 4), Some(4));
        // Empty string
        assert_eq!(find_marker("", 4), None);
    }
}
