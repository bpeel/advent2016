fn part1(nums: &[u32]) {
    let first = nums[0];

    let (_, part1): (u32, u32) =
        nums.iter().skip(1).fold((first, 0),
                                 |(prev, count), &next|
                                 (next, count + (next > prev) as u32));

    println!("part 1: {}", part1);
}

fn part2(nums: &[u32]) {
    let (a, b, c) = (nums[0], nums[1], nums[2]);

    fn fold((a, b, c, count): (u32, u32, u32, u32), &next: &u32)
            -> (u32, u32, u32, u32) {
        let part_sum = b + c;
        let prev_sum = a + part_sum;
        let next_sum = part_sum + next;

        (b, c, next, count + (next_sum > prev_sum) as u32)
    }

    let (_, _, _, part2): (u32, u32, u32, u32) =
        nums.iter().skip(3).fold((a, b, c, 0), fold);

    println!("part 2: {}", part2);
}

fn main() {
    let nums: Vec<u32> = std::io::stdin().lines().map(|line| {
        line.unwrap().trim_end().parse::<u32>().unwrap()
    }).collect();

    part1(&nums);
    part2(&nums);
}
