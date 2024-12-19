use std::process::ExitCode;

struct Layer {
    depth: u32,
    range: u32,
}

impl Layer {
    fn loop_length(&self) -> u32 {
        self.range + self.range.checked_sub(2).unwrap_or(0)
    }
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
            (layer.depth % layer.loop_length() == 0).then(|| {
                layer.depth * layer.range
            })
        })
        .sum::<u32>();

    println!("Part 1: {}", severity);

    // Part 2: For each layer, the delay+depth can not be a multiple
    // of the loop length.
    'delay_loop: for delay in 0.. {
        for layer in layers.iter() {
            if (delay + layer.depth) % layer.loop_length() == 0 {
                continue 'delay_loop;
            }
        }

        println!("Part 2: {}", delay);

        break;
    }

    ExitCode::SUCCESS
}
