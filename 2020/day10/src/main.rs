fn count_combinations(base_jolts: u32, nums: &[u32]) -> u64 {
    let Some((&first, tail)) = nums.split_first()
    else {
        return 0;
    };

    let Some(&next) = tail.first()
    else {
        return 1;
    };

    if next - base_jolts > 3 {
        count_combinations(first, tail)
    } else {
        count_combinations(base_jolts, tail)
            + count_combinations(first, tail)
    }
}

fn main() {
    let mut nums: Vec<u32> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    nums.sort_unstable();

    println!("part 2: {}", count_combinations(0, &nums));
}
