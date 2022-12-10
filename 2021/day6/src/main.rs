use std::io::Error;

fn main() -> Result<(), Error> {
    let mut contents = String::new();

    std::io::stdin().read_line(&mut contents)?;

    let mut fish_counts = vec![0u64; 9];

    for num in contents.trim_end().split(",") {
        let n = match num.parse::<usize>() {
            Err(e) => return Err(Error::new(std::io::ErrorKind::Other,
                                            format!("{}: {}", num, e))),
            Ok(n) => n,
        };

        if n >= fish_counts.len() {
            return Err(Error::new(std::io::ErrorKind::Other,
                                  format!("invalid fish value: {}", n)));
        }

        fish_counts[n] += 1;
    }

    for i in 0..256 {
        let new_fish = fish_counts[0];

        let len = fish_counts.len();
        fish_counts.copy_within(1..len, 0);
        fish_counts[6] += new_fish;
        fish_counts[len - 1] = new_fish;

        if i == 17 {
            println!("after 18 days: {}", fish_counts.iter().sum::<u64>());
        } else if i == 79 {
            println!("part 1: {}", fish_counts.iter().sum::<u64>());
        }
    }

    println!("part 2: {}", fish_counts.iter().sum::<u64>());

    Ok(())
}
