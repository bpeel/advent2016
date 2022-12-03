fn main() {
    let mut nums = std::io::stdin().lines().map(|line| {
        line.unwrap().trim_end().parse::<u32>().unwrap()
    });

    let first = nums.next().unwrap();

    let (_, part1): (u32, u32) = nums.fold((first, 0),
                                           |(prev, count), next|
                                           (next,
                                            count + (next > prev) as u32));

    println!("part 1: {}", part1);
}
