use std::collections::VecDeque;

fn queue_children(
    queue: &mut VecDeque::<usize>,
    start_index: usize,
    base_jolts: u32,
    nums: &[u32]
) {
    for index in start_index..nums.len() {
        if nums[index] - base_jolts <= 3 {
            queue.push_back(index);
        } else {
            break;
        }
    }
}

fn count_paths(nums: &[u32]) -> u64 {
    let mut queue = VecDeque::<usize>::new();

    queue_children(&mut queue, 0, 0, nums);

    let mut count = 0u64;

    while let Some(index) = queue.pop_front() {
        if index == nums.len() - 1 {
            count += 1;
        } else {
            queue_children(&mut queue, index + 1, nums[index], nums);
        }
    }

    count
}

fn main() {
    let mut nums: Vec<u32> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    nums.sort_unstable();

    println!("part 2: {}", count_paths(&nums));
}
