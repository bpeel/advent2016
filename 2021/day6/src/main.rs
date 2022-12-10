use std::io::Error;

fn main() -> Result<(), Error> {
    let mut contents = String::new();

    std::io::stdin().read_line(&mut contents)?;

    let mut fishes: Vec<u32> = contents.trim_end().split(",").map(|num| {
        num.parse::<u32>().unwrap()
    }).collect();

    for i in 0..80 {
        let mut new_fish = 0;

        for fish in fishes.iter_mut() {
            if *fish <= 0 {
                *fish = 6;
                new_fish += 1;
            } else {
                *fish -= 1;
            }
        }

        fishes.extend(std::iter::repeat(8u32).take(new_fish));

        if i == 17 {
            println!("after 18 days: {}", fishes.len());
        }
    }

    println!("part 1: {}", fishes.len());

    Ok(())
}
