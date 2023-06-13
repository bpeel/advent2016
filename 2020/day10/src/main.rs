fn count_paths(nums: &[u32]) -> u64 {
    let mut counts = Vec::with_capacity(nums.len());

    // Add all of the adaptors that can connect directly to the socket
    for &num in nums.iter() {
        if num <= 3 {
            counts.push(1);
        } else {
            break;
        }
    }

    counts.resize(nums.len(), 0);

    for (index, &num) in nums.iter().enumerate() {
        let count = counts[index];

        for index in index + 1..nums.len() {
            if nums[index] - num <= 3 {
                counts[index] += count;
            } else {
                break;
            }
        }
    }

    counts.last().map(|&x| x).unwrap_or(0)
}

fn main() {
    let mut nums: Vec<u32> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    nums.sort_unstable();

    println!("part 2: {}", count_paths(&nums));
}
