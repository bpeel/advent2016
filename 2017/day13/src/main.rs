use std::process::ExitCode;

struct Layer {
    depth: u32,
    range: u32,
}

fn read_layers() -> Result<Vec<Layer>, String> {
    let re = regex::Regex::new(r"^(\d+): (\d+)$").unwrap();
    let mut layers = Vec::new();

    for (line_num, result) in std::io::stdin().lines().enumerate() {
        let line = result.map_err(|e| e.to_string())?;

        let captures = re.captures(&line).ok_or_else(|| {
            format!("line {}: invalid syntax", line_num + 1)
        })?;

        let mut parts = [0; 2];

        for (i, part) in parts.iter_mut().enumerate() {
            let Ok(value) = captures[i + 1].parse::<u32>()
            else {
                return Err(format!(
                    "line {}: invalid number: {}",
                    line_num + 1,
                    &captures[i + 1],
                ));
            };

            *part = value;
        }

        layers.push(Layer { depth: parts[0], range: parts[1] });
    }

    Ok(layers)
}

fn main() -> ExitCode {
    let layers = match read_layers() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let severity = layers.iter()
        .filter_map(|layer| {
            let loop_length = layer.range +
                layer.range.checked_sub(2).unwrap_or(0);
            (layer.depth % loop_length == 0).then(|| {
                layer.depth * layer.range
            })
        })
        .sum::<u32>();

    println!("Part 1: {}", severity);

    ExitCode::SUCCESS
}
