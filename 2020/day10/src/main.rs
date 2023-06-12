struct StackEntry {
    count: u64,
    num: u32,
    past_next_child: usize,
}

fn count_paths(nums: &[u32]) -> u64 {
    let mut stack = vec![StackEntry {
        count: 0,
        num: match nums.last() {
            Some(&n) => n,
            None => return 0,
        },
        past_next_child: nums.len() - 1,
    }];

    let mut count = 0u64;

    while let Some(mut entry) = stack.pop() {
        match entry.past_next_child.checked_sub(1) {
            Some(child) if entry.num - nums[child] <= 3 => {
                entry.past_next_child = child;
                stack.push(entry);
                stack.push(StackEntry {
                    num: nums[child],
                    count: 0,
                    past_next_child: child,
                });
            },
            _ => {
                // Take into account connecting to the socket
                if entry.num <= 3 {
                    entry.count += 1;
                }

                match stack.last_mut() {
                    Some(last) => last.count += entry.count,
                    None => count += entry.count,
                }
            },
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
