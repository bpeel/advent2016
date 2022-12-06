fn find_marker(data: &str, marker_length: usize) -> Option<usize> {
    let bytes = data.as_bytes();
    let mut counts = [0u8; u8::MAX as usize + 1];
    let mut n_counts = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if counts[byte as usize] == 0 {
            n_counts += 1;
        }

        counts[byte as usize] += 1;

        if i >= marker_length {
            let old_byte = bytes[i - marker_length];

            if counts[old_byte as usize] <= 1 {
                n_counts -= 1;
            }

            counts[old_byte as usize] -= 1;
        }

        if n_counts >= marker_length {
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
