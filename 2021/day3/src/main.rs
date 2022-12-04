fn count_bits(nums: &[u32]) -> [u32; u32::BITS as usize] {
    let mut bit_counts = [0 as u32; u32::BITS as usize];

    for num in nums.iter() {
        let mut bits = *num;

        loop {
            let bit = bits.trailing_zeros();

            if bit >= u32::BITS {
                break;
            }

            bit_counts[bit as usize] += 1;
            bits &= !((1 as u32) << bit);
        }
    }

    bit_counts
}

fn leading_zeros(bit_counts: &[u32]) -> usize {
    bit_counts.iter().rev().take_while(|&&count| count == 0).count()
}

fn part1(nums: &[u32]) {
    let bit_counts = count_bits(nums);

    let gamma_rate =
        bit_counts.iter().enumerate().fold(0, |bits, (bit_num, &count)| {
            if count as usize > nums.len() / 2 {
                bits | ((1 as u32) << bit_num)
            } else {
                bits
            }
        });

    let leading_zeros = leading_zeros(&bit_counts);

    let epsilon_rate = (u32::MAX >> leading_zeros) ^ gamma_rate;

    println!("gamma = {}, epsilon = {}",
             gamma_rate,
             epsilon_rate);

    println!("part 1 = {}",
             gamma_rate * epsilon_rate);
}

fn reduce_nums(bit_count: usize,
               nums: &mut Vec<u32>,
               func: &mut dyn FnMut(usize, usize) -> bool)
{
    for i in (0..bit_count).rev() {
        if nums.len() <= 1 {
            break;
        }

        let one_count = nums
            .iter()
            .filter(|&&num| (num & (1u32 << i)) != 0)
            .count();
        let bit = func(nums.len() - one_count, one_count);

        let mut dst = 0;

        for num in 0..nums.len() {
            if ((nums[num] & (1u32 << i)) != 0) == bit {
                nums[dst] = nums[num];
                dst += 1;
            }
        }

        nums.truncate(dst);
    }
}

fn part2(nums: &[u32]) {
    let leading_zeros = leading_zeros(&count_bits(nums));
    let bit_count = u32::BITS as usize - leading_zeros;

    let mut oxygen_nums = nums.to_vec();

    reduce_nums(bit_count, &mut oxygen_nums, &mut |zero_count, one_count| {
        one_count >= zero_count
    });

    let mut co2_nums = nums.to_vec();

    reduce_nums(bit_count, &mut co2_nums, &mut |zero_count, one_count| {
        one_count < zero_count
    });

    println!("oxygen = {}, co2 = {}",
             oxygen_nums[0],
             co2_nums[0]);
    println!("part 2 = {}",
             oxygen_nums[0] * co2_nums[0]);
}

fn main() {
    let nums: Vec<u32> = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(line_num, result)| {
            match result {
                Err(..) => {
                    eprintln!("I/O error");
                    std::process::exit(1);
                },
                Ok(line) => match u32::from_str_radix(&line, 2) {
                    Err(e) => {
                        eprintln!("{}: {}", line_num + 1, e);
                        std::process::exit(1);
                    },
                    Ok(num) => num,
                }
            }
        })
        .collect();

    part1(&nums);
    part2(&nums);
}
