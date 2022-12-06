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
