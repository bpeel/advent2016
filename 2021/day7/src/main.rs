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

fn fuel_func_part1(distance: i32) -> i32 {
    distance
}

fn fuel_func_part2(distance: i32) -> i32 {
    distance * (distance + 1) / 2
}

fn calculate_fuel(crabs: &[i32],
                  target: i32,
                  fuel_func: fn(i32) -> i32) -> i32
{
    crabs.iter().map(|pos| fuel_func((pos - target).abs())).sum::<i32>()
}

fn find_best_target(crabs: &[i32],
                    fuel_func: fn(i32) -> i32) -> i32
{
    let last_crab = crabs.iter().max().unwrap();

    (0..last_crab + 1)
        .map(|pos| calculate_fuel(crabs, pos, fuel_func))
        .min()
        .unwrap()
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

    println!("part 1: {}", find_best_target(&crabs, fuel_func_part1));
    println!("part 2: {}", find_best_target(&crabs, fuel_func_part2));

    std::process::ExitCode::SUCCESS
}
