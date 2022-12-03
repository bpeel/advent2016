fn part1(nums: &[u32]) {
    let first = nums[0];

    let (_, part1): (u32, u32) =
        nums.iter().skip(1).fold((first, 0),
                                 |(prev, count), &next|
                                 (next, count + (next > prev) as u32));

    println!("part 1: {}", part1);
}

fn part2(nums: &[u32]) {
    let prev_sum = nums[0..3].iter().sum();

    let fold = |(prev_sum, count), (i, &next)| {
        let next_sum = prev_sum - nums[i - 3] + next;

        (next_sum, count + (next_sum > prev_sum) as u32)
    };

    let (_, part2): (u32, u32) =
        nums.iter().enumerate().skip(3).fold((prev_sum, 0), fold);

    println!("part 2: {}", part2);
}

fn main() {
    let nums: Vec<u32> = std::io::stdin().lines().map(|line| {
        line.unwrap().trim_end().parse::<u32>().unwrap()
    }).collect();

    part1(&nums);
    part2(&nums);
}
