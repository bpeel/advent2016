fn main() {
    let nums: Vec<u32> = std::io::stdin().lines().map(|line| {
        line.unwrap().trim_end().parse::<u32>().unwrap()
    }).collect();

    let first = nums[0];

    let (_, part1): (u32, u32) =
        nums.iter().skip(1).fold((first, 0),
                                 |(prev, count), &next|
                                 (next, count + (next > prev) as u32));

    println!("part 1: {}", part1);
}
