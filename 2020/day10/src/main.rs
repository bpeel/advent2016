fn build_tree(nums: &[u32]) -> Vec<Vec<usize>> {
    let mut nodes = vec![Vec::new(); nums.len()];

    // Add each adaptor as a child of the adaptors that it can be connected to
    for (index, &num) in nums.iter().enumerate() {
        for parent in index + 1..nums.len() {
            if nums[parent] - num <= 3 {
                nodes[parent].push(index);
            } else {
                break;
            }
        }
    }

    // Add special children for the plug socket
    for (index, &num) in nums.iter().enumerate() {
        if num <= 3 {
            nodes[index].push(usize::MAX);
        } else {
            break;
        }
    }

    nodes
}

struct StackEntry<'a> {
    count: u64,
    children: std::slice::Iter<'a, usize>,
}

fn count_paths(nodes: &Vec<Vec<usize>>) -> u64 {
    let mut stack = vec![StackEntry {
        count: 0,
        children: match nodes.last() {
            Some(last) => last.iter(),
            None => return 0,
        },
    }];

    let mut count = 0u64;

    while let Some(mut entry) = stack.pop() {
        match entry.children.next() {
            Some(&child) => {
                if child == usize::MAX {
                    entry.count += 1;
                    stack.push(entry);
                } else {
                    stack.push(entry);
                    stack.push(StackEntry {
                        count: 0,
                        children: nodes[child].iter(),
                    });
                }
            },
            None => {
                match stack.last_mut() {
                    Some(last) => last.count += entry.count,
                    None => count += entry.count,
                }
            },
        }
    }

    count
}

fn count_combinations(nums: &[u32]) -> u64 {
    count_paths(&build_tree(nums))
}

fn main() {
    let mut nums: Vec<u32> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    nums.sort_unstable();

    println!("part 2: {}", count_combinations(&nums));
}
