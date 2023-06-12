use std::collections::VecDeque;

#[derive(Clone)]
struct Count {
    left_count: u64,
    right_count: u64,
}

struct Counter<'a> {
    counts: Vec<Count>,
    left_queue: VecDeque::<usize>,
    right_queue: VecDeque::<usize>,
    nums: &'a [u32],
}

impl<'a> Counter<'a> {
    fn new(nums: &[u32]) -> Counter {
        Counter {
            counts: vec![Count { left_count: 0, right_count: 0}; nums.len()],
            left_queue: VecDeque::new(),
            right_queue: VecDeque::new(),
            nums,
        }
    }

    fn queue_children_from_left(
        &mut self,
        start_index: usize,
        base_jolts: u32,
    ) {
        for index in start_index..self.nums.len() {
            if self.nums[index] - base_jolts <= 3 {
                if self.counts[index].right_count == 0 {
                    self.counts[index].left_count += 1;
                    self.left_queue.push_back(index);
                }
            } else {
                break;
            }
        }
    }

    fn queue_children_from_right(
        &mut self,
        start_index: usize,
        base_jolts: u32,
    ) {
        for index in (0..start_index).rev() {
            if base_jolts - self.nums[index] <= 3 {
                if self.counts[index].left_count == 0 {
                    self.counts[index].right_count += 1;
                    self.right_queue.push_back(index);
                }
            } else {
                break;
            }
        }
    }
}

fn count_paths(nums: &[u32]) -> u64 {
    let mut counter = Counter::new(nums);

    counter.queue_children_from_left(0, 0);

    if let Some(last) = nums.len().checked_sub(1) {
        counter.right_queue.push_back(last);
        counter.counts[last].right_count += 1;
    } else {
        return 0;
    }

    while !counter.left_queue.is_empty() || !counter.right_queue.is_empty() {
        if let Some(index) = counter.left_queue.pop_front() {
            counter.queue_children_from_left(index + 1, nums[index]);
        }

        if let Some(index) = counter.right_queue.pop_front() {
            counter.queue_children_from_right(index, nums[index]);
        }
    }

    counter.counts.iter().map(|count| {
        if count.left_count > 0 && count.right_count > 0 {
            count.left_count * count.right_count
        } else {
            0
        }
    }).sum()
}

fn main() {
    let mut nums: Vec<u32> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    nums.sort_unstable();

    println!("part 2: {}", count_paths(&nums));
}
