use std::collections::HashSet;

fn find_marker(data: &str, marker_length: usize) -> Option<usize> {
    if data.len() < marker_length {
        return None;
    }

    let mut set = HashSet::<u8>::new();

    for i in 0..=data.len() - marker_length {
        set.clear();
        set.extend(&data.as_bytes()[i..i + marker_length]);

        if set.len() >= marker_length {
            return Some(i + marker_length);
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
