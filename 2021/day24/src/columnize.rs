fn main() {
    let mut columns = Vec::<Vec<String>>::new();
    let mut max_length = 1;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        max_length = std::cmp::max(line.len(), max_length);

        if line.starts_with("inp") {
            columns.push(Vec::new());
        }

        if let Some(lines) = columns.last_mut() {
            lines.push(line);
        }
    }

    let Some(n_lines) = columns.iter().map(Vec::len).max()
    else { return; };

    for line_num in 0..n_lines {
        for column in columns.iter() {
            let len = if let Some(line) = column.get(line_num) {
                print!("{}", line);
                line.len()
            } else {
                0
            };

            for _ in len..=max_length {
                print!(" ");
            }
        }

        println!();
    }
}
