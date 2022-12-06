use std::collections::HashMap;

fn find_marker(data: &str, marker_length: usize) -> Option<usize> {
    let bytes = data.as_bytes();
    let mut counts = HashMap::<u8, u32>::new();

    for (i, &byte) in bytes.iter().enumerate() {
        counts.entry(byte).and_modify(|count| *count += 1).or_insert(1);

        if i >= marker_length {
            let old_byte = bytes[i - marker_length];
            let old_count = counts.get_mut(&old_byte).unwrap();

            if *old_count <= 1 {
                counts.remove(&old_byte);
            } else {
                *old_count -= 1;
            }
        }

        if counts.len() >= marker_length {
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
