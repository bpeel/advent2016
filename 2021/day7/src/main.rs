fn read_list<T>() -> Result<Vec<T>, std::io::Error>
    where T: std::str::FromStr
{
    let mut result = Vec::new();

    let mut contents = String::new();

    std::io::stdin().read_line(&mut contents)?;

    for part in contents.trim_end().split(",") {
        let part = match part.parse::<T>() {
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                               format!("invalid value: {}",
                                                       part)));
            },
            Ok(n) => n,
        };

        result.push(part);
    }

    Ok(result)
}

fn calculate_fuel(crabs: &[i32], target: i32) -> i32 {
    crabs.iter().map(|pos| (pos - target).abs()).sum::<i32>()
}

fn main() -> std::process::ExitCode {
    let crabs = match read_list::<i32>() {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(crabs) => crabs,
    };

    if crabs.len() == 0 {
        eprintln!("Empty crab list");
        return std::process::ExitCode::FAILURE;
    }

    let best_fuel_used = crabs
        .iter()
        .map(|&crab| calculate_fuel(&crabs, crab))
        .min()
        .unwrap();

    println!("part 1: {}", best_fuel_used);

    std::process::ExitCode::SUCCESS
}
